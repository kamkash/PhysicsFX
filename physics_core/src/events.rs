use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEventType {
    PointerDown,
    PointerUp,
    PointerMove,
    KeyDown,
    KeyUp,
    // Add more as needed
}

#[derive(Debug, Clone, Copy)]
pub struct GameEvent {
    pub event_type: InputEventType,
    pub x: f32, // For pointer events
    pub y: f32, // For pointer events
    pub key_code: Option<i32>, // For keyboard events
                // timestamp?
}

impl GameEvent {
    pub fn new_pointer(event_type: InputEventType, x: f32, y: f32) -> Self {
        Self {
            event_type,
            x,
            y,
            key_code: None,
        }
    }

    pub fn new_key(event_type: InputEventType, key_code: i32) -> Self {
        Self {
            event_type,
            x: -1.0,
            y: -1.0,
            key_code: Some(key_code),
        }
    }
}

/// Resource to store events that happened in the current frame (or since last flush)
#[derive(Resource, Default)]
pub struct EventQueue {
    pub events: Vec<GameEvent>,
}

impl EventQueue {
    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, GameEvent> {
        self.events.drain(..)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
