package app.kamkash.physicsfx

actual object NativeLib {
    actual fun getInfo(): String {
        return try {
            PhysicsCore.wasm_get_info()
        } catch (e: Throwable) {
            "WASM initializing..."
        }
    }

    actual fun setGravity(y: Float) {
        PhysicsCore.wasm_set_gravity(y)
    }

    actual fun setTimeScale(scale: Float) {
        PhysicsCore.wasm_set_time_scale(scale)
    }

    actual fun setPaused(paused: Boolean) {
        PhysicsCore.wasm_set_paused(paused)
    }

    actual fun resetSimulation() {
        PhysicsCore.wasm_reset_simulation()
    }

    actual fun onPointerEvent(eventType: Int, x: Float, y: Float, button: Int) {
        PhysicsCore.wasm_on_pointer_event(eventType, x, y, button)
    }

    actual fun onKeyEvent(eventType: Int, keyCode: Int) {
        PhysicsCore.wasm_on_key_event(eventType, keyCode)
    }
}
