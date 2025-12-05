#ifndef PHYSICS_CORE_H
#define PHYSICS_CORE_H

#include <stdint.h>
#include <stdbool.h>

const char* physics_core_get_info();
void physics_core_free_string(char* s);

// Game loop lifecycle
bool wgpu_init(int32_t width, int32_t height);
void wgpu_update(float delta_time);
void wgpu_render();
void wgpu_resize(int32_t width, int32_t height);
void wgpu_shutdown();

#endif
