//! Integration tests for Strategy Pattern movement components

use physics_core::{
    AnimatorComponent, CircularMovement, HorizontalRandomMovement, LinearMovement,
    MovementStrategy, SinusoidalMovement, SpriteSheetComponent,
};

#[test]
fn test_linear_movement() {
    let strategy = LinearMovement {
        velocity_x: 1.0,
        velocity_y: 0.5,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), 2.0);
    assert!((x - 2.0).abs() < 0.001, "x should be 2.0, got {}", x);
    assert!((y - 1.0).abs() < 0.001, "y should be 1.0, got {}", y);
}

#[test]
fn test_linear_movement_with_origin() {
    let strategy = LinearMovement {
        velocity_x: 1.0,
        velocity_y: 1.0,
    };
    let (x, y) = strategy.calculate_position((5.0, 5.0), 3.0);
    assert!((x - 8.0).abs() < 0.001, "x should be 8.0, got {}", x);
    assert!((y - 8.0).abs() < 0.001, "y should be 8.0, got {}", y);
}

#[test]
fn test_sinusoidal_movement_at_zero() {
    let strategy = SinusoidalMovement {
        amplitude: 1.0,
        frequency: 1.0,
        direction_x: 1.0,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), 0.0);
    assert!((x - 0.0).abs() < 0.001, "x should be 0.0, got {}", x);
    assert!((y - 0.0).abs() < 0.001, "y should be 0.0, got {}", y);
}

#[test]
fn test_sinusoidal_movement_at_pi_half() {
    use std::f32::consts::PI;
    let strategy = SinusoidalMovement {
        amplitude: 1.0,
        frequency: 1.0,
        direction_x: 0.0,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), PI / 2.0);
    assert!((x - 0.0).abs() < 0.001, "x should be 0.0, got {}", x);
    assert!(
        (y - 1.0).abs() < 0.001,
        "y should be ~1.0 (sin(PI/2)), got {}",
        y
    );
}

#[test]
fn test_circular_movement_at_zero() {
    let strategy = CircularMovement {
        radius: 1.0,
        angular_speed: 1.0,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), 0.0);
    assert!(
        (x - 1.0).abs() < 0.001,
        "x should be 1.0 (cos(0)), got {}",
        x
    );
    assert!(
        (y - 0.0).abs() < 0.001,
        "y should be 0.0 (sin(0)), got {}",
        y
    );
}

#[test]
fn test_circular_movement_at_pi_half() {
    use std::f32::consts::PI;
    let strategy = CircularMovement {
        radius: 1.0,
        angular_speed: 1.0,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), PI / 2.0);
    assert!(x.abs() < 0.001, "x should be ~0.0 (cos(PI/2)), got {}", x);
    assert!(
        (y - 1.0).abs() < 0.001,
        "y should be ~1.0 (sin(PI/2)), got {}",
        y
    );
}

#[test]
fn test_animator_component_default() {
    let animator = AnimatorComponent::default();
    assert!(animator.is_playing);
    assert!((animator.elapsed_time - 0.0).abs() < 0.001);
    assert!((animator.speed - 1.0).abs() < 0.001);
    assert_eq!(animator.current_frame, 0);
}

#[test]
fn test_strategy_names() {
    let linear = LinearMovement {
        velocity_x: 1.0,
        velocity_y: 1.0,
    };
    assert_eq!(linear.name(), "Linear");

    let sinusoidal = SinusoidalMovement {
        amplitude: 1.0,
        frequency: 1.0,
        direction_x: 1.0,
    };
    assert_eq!(sinusoidal.name(), "Sinusoidal");

    let circular = CircularMovement {
        radius: 1.0,
        angular_speed: 1.0,
    };
    assert_eq!(circular.name(), "Circular");
}

#[test]
fn test_horizontal_random_movement_at_zero() {
    let strategy = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    let (x, y) = strategy.calculate_position((0.0, 0.0), 0.0);
    // At t=0, should be at origin
    assert!((x - 0.0).abs() < 0.001, "x should be 0.0, got {}", x);
    assert!(
        (y - 0.0).abs() < 0.001,
        "y should be 0.0 (no vertical movement), got {}",
        y
    );
}

#[test]
fn test_horizontal_random_movement_during_first_segment() {
    let strategy = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    // At t=1.0, should have moved in some direction for 1 second
    let (x, y) = strategy.calculate_position((0.0, 0.0), 1.0);
    // Should have moved horizontally (not at origin)
    assert!((x.abs() - 1.0).abs() < 0.001, "x should be ±1.0, got {}", x);
    assert!((y - 0.0).abs() < 0.001, "y should be 0.0, got {}", y);
}

#[test]
fn test_horizontal_random_movement_during_pause() {
    let strategy = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    // At t=2.5, should be in pause phase (cycle = 3s, first 2s = move, last 1s = pause)
    let pos_at_2 = strategy.calculate_position((0.0, 0.0), 2.0);
    let pos_at_2_5 = strategy.calculate_position((0.0, 0.0), 2.5);
    // Position should be same during pause
    assert!(
        (pos_at_2.0 - pos_at_2_5.0).abs() < 0.001,
        "x should not change during pause, was {} at t=2, {} at t=2.5",
        pos_at_2.0,
        pos_at_2_5.0
    );
}

#[test]
fn test_horizontal_random_movement_name() {
    let strategy = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    assert_eq!(strategy.name(), "HorizontalRandom");
}

#[test]
fn test_horizontal_random_movement_deterministic() {
    let strategy1 = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    let strategy2 = HorizontalRandomMovement {
        speed: 1.0,
        move_duration: 2.0,
        pause_duration: 1.0,
        seed: 42,
    };
    // Same seed should produce same results
    for t in [0.0, 1.0, 3.5, 7.0, 10.0] {
        let (x1, y1) = strategy1.calculate_position((0.0, 0.0), t);
        let (x2, y2) = strategy2.calculate_position((0.0, 0.0), t);
        assert!(
            (x1 - x2).abs() < 0.001,
            "Same seed should produce same x at t={}: {} vs {}",
            t,
            x1,
            x2
        );
        assert!((y1 - y2).abs() < 0.001);
    }
}

// --- SpriteSheetComponent Tests ---

#[test]
fn test_sprite_sheet_default() {
    let sheet = SpriteSheetComponent::default();
    assert_eq!(sheet.columns, 1);
    assert_eq!(sheet.rows, 1);
    assert_eq!(sheet.frame_count, 1);
    assert!((sheet.frame_duration - 0.1).abs() < 0.001);
    assert!(sheet.looping);
}

#[test]
fn test_sprite_sheet_uv_single_frame() {
    let sheet = SpriteSheetComponent::new(1, 1, 1, 0.1, true);
    let (u, v, uw, vh) = sheet.uv_for_frame(0);
    assert!((u - 0.0).abs() < 0.001, "u should be 0.0, got {}", u);
    assert!((v - 0.0).abs() < 0.001, "v should be 0.0, got {}", v);
    assert!(
        (uw - 1.0).abs() < 0.001,
        "u_width should be 1.0, got {}",
        uw
    );
    assert!(
        (vh - 1.0).abs() < 0.001,
        "v_height should be 1.0, got {}",
        vh
    );
}

#[test]
fn test_sprite_sheet_uv_4x4_grid() {
    let sheet = SpriteSheetComponent::new(4, 4, 16, 0.1, true);

    // Frame 0 (top-left)
    let (u, v, uw, vh) = sheet.uv_for_frame(0);
    assert!((u - 0.0).abs() < 0.001);
    assert!((v - 0.0).abs() < 0.001);
    assert!((uw - 0.25).abs() < 0.001);
    assert!((vh - 0.25).abs() < 0.001);

    // Frame 1 (second column, first row)
    let (u, v, _, _) = sheet.uv_for_frame(1);
    assert!(
        (u - 0.25).abs() < 0.001,
        "frame 1 u should be 0.25, got {}",
        u
    );
    assert!((v - 0.0).abs() < 0.001);

    // Frame 4 (first column, second row)
    let (u, v, _, _) = sheet.uv_for_frame(4);
    assert!((u - 0.0).abs() < 0.001);
    assert!(
        (v - 0.25).abs() < 0.001,
        "frame 4 v should be 0.25, got {}",
        v
    );

    // Frame 15 (last frame, bottom-right)
    let (u, v, _, _) = sheet.uv_for_frame(15);
    assert!(
        (u - 0.75).abs() < 0.001,
        "frame 15 u should be 0.75, got {}",
        u
    );
    assert!(
        (v - 0.75).abs() < 0.001,
        "frame 15 v should be 0.75, got {}",
        v
    );
}

#[test]
fn test_sprite_sheet_frame_for_time_looping() {
    let sheet = SpriteSheetComponent::new(4, 1, 4, 0.5, true); // 4 frames, 0.5s each

    assert_eq!(sheet.frame_for_time(0.0, 1.0), 0);
    assert_eq!(sheet.frame_for_time(0.25, 1.0), 0); // Still in frame 0
    assert_eq!(sheet.frame_for_time(0.5, 1.0), 1); // Frame 1
    assert_eq!(sheet.frame_for_time(1.5, 1.0), 3); // Frame 3
    assert_eq!(sheet.frame_for_time(2.0, 1.0), 0); // Loop back to frame 0
    assert_eq!(sheet.frame_for_time(3.0, 1.0), 2); // Loop continues
}

#[test]
fn test_sprite_sheet_frame_for_time_not_looping() {
    let sheet = SpriteSheetComponent::new(4, 1, 4, 0.5, false); // 4 frames, not looping

    assert_eq!(sheet.frame_for_time(0.0, 1.0), 0);
    assert_eq!(sheet.frame_for_time(1.5, 1.0), 3); // Last frame
    assert_eq!(sheet.frame_for_time(10.0, 1.0), 3); // Stays on last frame
}

#[test]
fn test_sprite_sheet_frame_for_time_with_speed() {
    let sheet = SpriteSheetComponent::new(4, 1, 4, 0.5, true);

    // At 2x speed, 0.25s real time = 0.5s animation time = frame 1
    assert_eq!(sheet.frame_for_time(0.25, 2.0), 1);

    // At 0.5x speed, 1.0s real time = 0.5s animation time = frame 1
    assert_eq!(sheet.frame_for_time(1.0, 0.5), 1);
}
