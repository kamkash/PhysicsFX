use bevy_ecs::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct AnimatorComponent {
    pub elapsed_time: f32,
    pub is_playing: bool,
    pub current_frame: u32,
    pub speed: f32,
}

impl Default for AnimatorComponent {
    fn default() -> Self {
        Self {
            elapsed_time: 0.0,
            is_playing: true,
            current_frame: 0,
            speed: 1.0,
        }
    }
}

pub fn animation_system(
    mut query: Query<(&mut AnimatorComponent, &crate::sprite::SpriteSheetComponent)>,
    // In a real Bevy app we'd use Res<Time>, but here we pass dt manually or use a resource.
    // For now we assume the caller handles dt or we use a Global dt.
    dt: f32,
) {
    for (mut animator, sprite_sheet) in query.iter_mut() {
        if animator.is_playing {
            animator.elapsed_time += dt;
            animator.current_frame =
                sprite_sheet.frame_for_time(animator.elapsed_time, animator.speed);
        }
    }
}
