use harmonius_integration::{
    AUDIO_COMMANDS_MPSC_CAPACITY, GAME_LOOP_PHASE_COUNT, INPUT_EVENTS_MPSC_CAPACITY,
    IO_REQUESTS_MPSC_CAPACITY, NETWORK_PACKETS_MPSC_CAPACITY, RENDER_FRAME_TRIPLE_BUFFER_SLOTS,
    SAVE_WRITES_MPSC_CAPACITY, game_loop_phases_ordered, GameLoopPhase,
};

#[test]
fn game_loop_phase_count_is_eight() {
    assert_eq!(GAME_LOOP_PHASE_COUNT, 8);
}

#[test]
fn game_loop_phases_run_one_through_eight_without_gaps() {
    let phases = game_loop_phases_ordered();
    for (idx, phase) in phases.iter().enumerate() {
        assert_eq!(*phase as u8, (idx + 1) as u8);
    }
}

#[test]
fn first_phase_is_input_last_is_frame_end() {
    let phases = game_loop_phases_ordered();
    assert_eq!(phases[0], GameLoopPhase::InputProcessing);
    assert_eq!(phases[7], GameLoopPhase::FrameEnd);
}

#[test]
fn channel_capacities_match_high_level_design_table() {
    assert_eq!(INPUT_EVENTS_MPSC_CAPACITY, 1024);
    assert_eq!(NETWORK_PACKETS_MPSC_CAPACITY, 4096);
    assert_eq!(RENDER_FRAME_TRIPLE_BUFFER_SLOTS, 3);
    assert_eq!(AUDIO_COMMANDS_MPSC_CAPACITY, 2048);
    assert_eq!(IO_REQUESTS_MPSC_CAPACITY, 1024);
    assert_eq!(SAVE_WRITES_MPSC_CAPACITY, 64);
}
