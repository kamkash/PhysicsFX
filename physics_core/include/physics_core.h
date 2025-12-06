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

#endif
