//! Harmonius triangle demo — main + render threads.

#![deny(clippy::all)]

mod shaders;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam_channel::{bounded, Receiver};
use harmonius_core::EngineError;
use harmonius_gpu::GpuDevice;
use harmonius_platform::{SurfaceEvent, Window, WindowConfig, WindowEvent};

use crate::shaders::load_spirv;

fn main() {
    if let Err(err) = run() {
        eprintln!("harmonius_app error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), EngineError> {
    let mut window = Window::new(WindowConfig::default())?;
    let surface = window.surface_handle();
    let (resize_tx, resize_rx) = bounded::<SurfaceEvent>(8);
    let running = Arc::new(AtomicBool::new(true));
    let running_render = Arc::clone(&running);

    let vert_spirv = load_spirv("triangle.vert");
    let frag_spirv = load_spirv("triangle.frag");

    let render_handle = thread::spawn(move || {
        if let Err(err) = render_loop(surface, &vert_spirv, &frag_spirv, resize_rx, running_render)
        {
            eprintln!("render thread error: {err}");
        }
    });

    while running.load(Ordering::Relaxed) {
        let events = window.poll_events()?;
        for event in &events {
            match event {
                WindowEvent::CloseRequested => {
                    running.store(false, Ordering::Relaxed);
                }
                WindowEvent::Resized(size) => {
                    let _ = resize_tx.send(SurfaceEvent::Resized(*size));
                }
                WindowEvent::FocusChanged(_) => {}
            }
        }
        if !running.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_millis(8));
    }

    let _ = render_handle.join();
    Ok(())
}

fn render_loop(
    surface: harmonius_platform::SurfaceHandle,
    vert_spirv: &[u32],
    frag_spirv: &[u32],
    resize_rx: Receiver<SurfaceEvent>,
    running: Arc<AtomicBool>,
) -> Result<(), EngineError> {
    let mut gpu = GpuDevice::new(surface, vert_spirv, frag_spirv)
        .map_err(|e| EngineError::Gpu(e.to_string()))?;
    while running.load(Ordering::Relaxed) {
        while let Ok(event) = resize_rx.try_recv() {
            if let SurfaceEvent::Resized(size) = event {
                gpu.resize(size.width, size.height)
                    .map_err(|e| EngineError::Gpu(e.to_string()))?;
            }
        }
        if let Err(err) = gpu.draw_triangle_frame() {
            let msg = err.to_string();
            if msg.contains("out of date") {
                continue;
            }
            return Err(EngineError::Gpu(msg));
        }
        thread::sleep(Duration::from_millis(1));
    }
    Ok(())
}
