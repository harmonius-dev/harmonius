use harmonius_gpu_runtime::{
    assemble_gpu_frame_stats, CommandBuffer, DrawListStats, FrameCollector, GpuAllocatorStats,
    GpuFrameStats, GpuProfilerState, GpuScope, ProfilingQueries, RenderPhase, ResolvedTimestamps,
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
