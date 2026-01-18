use bevy_ecs::prelude::*;
use bevy_sprite::prelude::*;

#[derive(Component)]
pub struct SpriteSample {
    pub rotate_speed: f32,
}

impl Default for SpriteSample {
    fn default() -> Self {
        Self { rotate_speed: 1.0 }
    }
}

#[derive(Resource)]
pub struct GameTime(pub f32);

// Example system that might query for Bevy Sprites
// Note: Actual rendering happens in loose integration with WGPU,
// so this system just demonstrates data manipulation on standard Bevy components.
pub fn sprite_spin_system(mut query: Query<(&mut Sprite, &SpriteSample)>, time: Res<GameTime>) {
    let elapsed = time.0;
    for (mut sprite, sample) in query.iter_mut() {
        // Just an example of accessing fields
        if let Some(ref mut size) = sprite.custom_size {
            size.x = (elapsed * sample.rotate_speed).sin();
        }
    }
}
