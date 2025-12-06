package app.kamkash.physicsfx

import kotlin.js.Promise
import kotlin.js.JsAny

// External declarations for WASM Rust bindings
@JsModule("./physics_core.js")
external object PhysicsCore {
    @JsName("default")
    fun init(): Promise<JsAny>
    
    fun wasm_init(canvasId: String, width: Int, height: Int): Promise<JsAny>
    fun wasm_update(deltaTime: Float)
    fun wasm_render()
    fun wasm_resize(width: Int, height: Int)
    fun wasm_shutdown()
    fun wasm_get_info(): String
}
