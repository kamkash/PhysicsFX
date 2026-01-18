use bevy_animation::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct AnimationSample {
    pub speed: f32,
    pub paused: bool,
}

impl Default for AnimationSample {
    fn default() -> Self {
        Self {
            speed: 1.0,
            paused: false,
        }
    }
}

pub fn animation_control_system(mut query: Query<(&mut AnimationPlayer, &AnimationSample)>) {
    for (_player, sample) in query.iter_mut() {
        if sample.paused {
            // Logic to pause would go here (requires active animation handle)
            // println!("Animation paused");
        } else {
            // Logic to resume/speed up would go here
            // println!("Animation playing at speed {}", sample.speed);
        }
    }
}
