package app.kamkash.physicsfx

actual object NativeLib {
    actual fun getInfo(): String {
        return "WASM Ready (Rust call disabled)"
        /*
        return try {
            PhysicsCore.wasm_get_info()
        } catch (e: Exception) {
            "WASM initializing... (see console for details)"
        }
        */
    }
}
