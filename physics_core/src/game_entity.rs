//! Strategy Pattern Components for Animated Entities
//!
//! This module provides components and traits for implementing animated game entities
//! with different movement patterns using the Strategy Pattern.

use bevy_ecs::prelude::*;

// --- Marker Component ---

/// Marker component for animated game entities
#[derive(Component, Clone, Copy, Default)]
pub struct GameEntity;

/// Marker component for entities that can be controlled by player input
#[derive(Component, Clone, Copy, Default)]
pub struct Controllable;

// --- Movement Strategy Pattern ---

/// Strategy pattern trait for different movement behaviors (IMovementStrategy)
pub trait MovementStrategy: Send + Sync {
    /// Calculate the new position based on origin and elapsed time
    fn calculate_position(&self, origin: (f32, f32), elapsed_time: f32) -> (f32, f32);

    /// Get a descriptive name for this strategy
    fn name(&self) -> &'static str;
}

/// Linear movement in a constant direction
pub struct LinearMovement {
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl MovementStrategy for LinearMovement {
    fn calculate_position(&self, origin: (f32, f32), elapsed_time: f32) -> (f32, f32) {
        (
            origin.0 + self.velocity_x * elapsed_time,
            origin.1 + self.velocity_y * elapsed_time,
        )
    }

    fn name(&self) -> &'static str {
        "Linear"
    }
}

/// Sinusoidal wave movement
pub struct SinusoidalMovement {
    pub amplitude: f32,
    pub frequency: f32,
    pub direction_x: f32, // Primary movement direction
}

impl MovementStrategy for SinusoidalMovement {
    fn calculate_position(&self, origin: (f32, f32), elapsed_time: f32) -> (f32, f32) {
        let x = origin.0 + self.direction_x * elapsed_time;
        let y = origin.1 + self.amplitude * (self.frequency * elapsed_time).sin();
        (x, y)
    }

    fn name(&self) -> &'static str {
        "Sinusoidal"
    }
}

/// Circular orbit movement
pub struct CircularMovement {
    pub radius: f32,
    pub angular_speed: f32,
}

impl MovementStrategy for CircularMovement {
    fn calculate_position(&self, origin: (f32, f32), elapsed_time: f32) -> (f32, f32) {
        let angle = self.angular_speed * elapsed_time;
        (
            origin.0 + self.radius * angle.cos(),
            origin.1 + self.radius * angle.sin(),
        )
    }

    fn name(&self) -> &'static str {
        "Circular"
    }
}

/// Horizontal random movement with pauses and direction changes
///
/// Moves horizontally in a straight line, pauses, then changes direction.
/// Uses deterministic pseudo-random behavior based on elapsed time.
pub struct HorizontalRandomMovement {
    /// Speed of horizontal movement (units per second)
    pub speed: f32,
    /// Duration of each movement segment (seconds)
    pub move_duration: f32,
    /// Duration of pause between movements (seconds)
    pub pause_duration: f32,
    /// Seed value for pseudo-random direction selection
    pub seed: u32,
}

impl HorizontalRandomMovement {
    /// Simple hash function for deterministic pseudo-random direction
    fn direction_for_segment(&self, segment: u32) -> f32 {
        // Use a simple hash combining seed and segment number
        let hash = self
            .seed
            .wrapping_mul(2654435761)
            .wrapping_add(segment.wrapping_mul(1597334677));
        // Return -1.0 or 1.0 based on lowest bit
        if hash % 2 == 0 {
            1.0
        } else {
            -1.0
        }
    }
}

impl MovementStrategy for HorizontalRandomMovement {
    fn calculate_position(&self, origin: (f32, f32), elapsed_time: f32) -> (f32, f32) {
        let cycle_duration = self.move_duration + self.pause_duration;

        // Determine which cycle we're in and the time within that cycle
        let current_cycle = (elapsed_time / cycle_duration) as u32;
        let time_in_cycle = elapsed_time % cycle_duration;

        // Calculate cumulative displacement from all previous complete cycles
        let mut total_displacement = 0.0f32;
        for seg in 0..current_cycle {
            let dir = self.direction_for_segment(seg);
            total_displacement += dir * self.speed * self.move_duration;
        }

        // Add displacement from current cycle (only during movement phase, not pause)
        if time_in_cycle < self.move_duration {
            let current_dir = self.direction_for_segment(current_cycle);
            total_displacement += current_dir * self.speed * time_in_cycle;
        } else {
            // During pause phase, use full movement from current segment
            let current_dir = self.direction_for_segment(current_cycle);
            total_displacement += current_dir * self.speed * self.move_duration;
        }

        (origin.0 + total_displacement, origin.1)
    }

    fn name(&self) -> &'static str {
        "HorizontalRandom"
    }
}

// --- Movement Component ---

/// Component holding the movement strategy for an entity
pub struct MovementComponent {
    /// The active movement strategy
    pub strategy: Box<dyn MovementStrategy>,
    /// Starting origin position (used as reference for calculations)
    pub origin: (f32, f32),
}

// SAFETY: MovementStrategy requires Send + Sync, so Box<dyn MovementStrategy> is Send + Sync
unsafe impl Send for MovementComponent {}
unsafe impl Sync for MovementComponent {}

// Implement Component manually since we can't derive it with Box<dyn Trait>
impl Component for MovementComponent {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
}
