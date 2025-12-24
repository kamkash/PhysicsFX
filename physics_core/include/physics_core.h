#ifndef PHYSICS_CORE_H
#define PHYSICS_CORE_H

#include <stdint.h>
#include <stdbool.h>

const char* physics_core_get_info();
void physics_core_free_string(char* s);

// Game loop lifecycle
// surface_handle: Platform-specific native surface handle
//   - iOS: CAMetalLayer*
//   - macOS: NSView*
//   - Windows: HWND
//   - Linux: X11 Window
//   - Android: ANativeWindow*
bool wgpu_init(void* surface_handle, int32_t width, int32_t height);
void wgpu_update(float delta_time);
void wgpu_render();
void wgpu_resize(int32_t width, int32_t height);
void wgpu_shutdown();

// Simulation controls
void physics_core_set_gravity(float y);
void physics_core_set_time_scale(float scale);
void physics_core_set_paused(bool paused);
void physics_core_reset_simulation();
void physics_core_on_pointer_event(int32_t event_type, float x, float y, int32_t button);
void physics_core_on_key_event(int32_t event_type, int32_t key_code);

#endif
