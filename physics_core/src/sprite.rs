use bevy_ecs::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct SpriteSheetComponent {
    pub rows: u32,
    pub columns: u32,
    pub frame_count: u32,
    pub frame_duration: f32, // in seconds
    pub looping: bool,
}

impl SpriteSheetComponent {
    pub fn new(
        rows: u32,
        columns: u32,
        frame_count: u32,
        frame_duration: f32,
        looping: bool,
    ) -> Self {
        Self {
            rows,
            columns,
            frame_count,
            frame_duration,
            looping,
        }
    }

    pub fn frame_for_time(&self, elapsed: f32, speed: f32) -> u32 {
        if self.frame_count == 0 || self.frame_duration <= 0.0 {
            return 0;
        }
        let total_duration = self.frame_duration * self.frame_count as f32;
        let adjusted_elapsed = elapsed * speed;

        if self.looping {
            let cycle_time = adjusted_elapsed % total_duration;
            (cycle_time / self.frame_duration) as u32 % self.frame_count
        } else {
            let frame = (adjusted_elapsed / self.frame_duration) as u32;
            frame.min(self.frame_count - 1)
        }
    }

    pub fn uv_for_frame(&self, frame: u32) -> (f32, f32, f32, f32) {
        let frame = frame % self.frame_count;
        let row = frame / self.columns;
        let col = frame % self.columns;

        let width = 1.0 / self.columns as f32;
        let height = 1.0 / self.rows as f32;
        let u = col as f32 * width;
        let v = row as f32 * height;

        (u, v, width, height)
    }
}

impl Default for SpriteSheetComponent {
    fn default() -> Self {
        Self {
            rows: 1,
            columns: 1,
            frame_count: 1,
            frame_duration: 0.1,
            looping: true,
        }
    }
}
