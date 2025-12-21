package app.kamkash.physicsfx

import kotlin.js.JsAny
import kotlin.js.Promise

// External declarations for WASM Rust bindings
@JsModule("./physics_core.js")
external object PhysicsCore {
    @JsName("default") fun init(): Promise<JsAny>

    fun wasm_init(canvasId: String, width: Int, height: Int): Promise<JsAny>
    fun wasm_update(deltaTime: Float)
    fun wasm_render()
    fun wasm_resize(width: Int, height: Int)
    fun wasm_shutdown()
    fun wasm_get_info(): String
    fun wasm_set_gravity(y: Float)
    fun wasm_set_time_scale(scale: Float)
    fun wasm_set_paused(paused: Boolean)
    fun wasm_reset_simulation()
}
