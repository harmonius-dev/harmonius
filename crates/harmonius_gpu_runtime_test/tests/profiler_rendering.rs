use harmonius_gpu_runtime::{
    assemble_gpu_frame_stats, CommandBuffer, DrawListStats, FrameCollector, GpuAllocatorStats,
    GpuFrameStats, GpuProfilerState, GpuPassTiming, GpuScope, ProfilingQueries, RenderPhase,
    ResolvedTimestamps,
};

#[test]
fn tc_ir_5_7_f_1_mpsc_handoff_resolved_timestamps_drained() {
    let mut collector = FrameCollector::new(0, 4);
    let sender = collector.sender();

    for frame_number in 1..=5_u64 {
        sender.send_drop_oldest(ResolvedTimestamps {
            frame_number,
            timings: Vec::new(),
            stats: GpuFrameStats::default(),
            per_view: Vec::new(),
        });
    }

    let drained = collector.drain_gpu().expect("latest message");
    assert_eq!(drained.frame_number, 5);

    let capture = collector.assemble_gpu(drained);
    assert_eq!(capture.frame_number, 5);
}

#[test]
fn tc_ir_5_7_2_1_draw_call_count_rollup_matches_design_helpers() {
    let per_view = vec![
        DrawListStats {
            view_id: 0,
            phase: RenderPhase::GBuffer,
            draw_calls: 50,
            triangles: 0,
        },
        DrawListStats {
            view_id: 1,
            phase: RenderPhase::Transparent,
            draw_calls: 10,
            triangles: 0,
        },
    ];
    let vram = GpuAllocatorStats::default();
    let stats = assemble_gpu_frame_stats(&per_view, &vram);
    assert_eq!(stats.draw_calls, 60);
}

#[test]
fn tc_ir_5_7_7_1_runtime_toggle_off_allocates_no_queries() {
    let state = GpuProfilerState::new(8);
    let mut cmd = CommandBuffer::default();
    let mut pool = ProfilingQueries::new(8, 1.0);
    {
        let _scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassA");
    }
    assert!(cmd.ops.is_empty());
    assert_eq!(pool.allocated_pair_count(), 0);
}

#[test]
fn tc_ir_5_7_4_2_two_frame_readback_fake_pool() {
    let state = GpuProfilerState::new(8);
    state.enabled.store(true, std::sync::atomic::Ordering::Relaxed);
    let mut cmd = CommandBuffer::default();
    let mut pool = ProfilingQueries::new(8, 1.0);
    pool.set_readback_latency_frames(2);

    pool.reset();
    {
        let _scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassA");
    }
    assert!(
        pool.read_resolved().is_empty(),
        "readback should not be ready on the capture frame"
    );

    pool.reset();
    {
        let _scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassB");
    }
    assert!(
        pool.read_resolved().is_empty(),
        "one frame of latency is still not enough"
    );

    pool.reset();
    {
        let _scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassC");
    }
    let timings = pool.read_resolved();
    assert_eq!(
        timings,
        vec![GpuPassTiming {
            pass_id: 0,
            pass_name: "PassA",
            begin_ms: 1.0,
            end_ms: 5.0,
            duration_ms: 4.0,
        }]
    );
}
