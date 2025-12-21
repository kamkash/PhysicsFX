package app.kamkash.physicsfx

actual object NativeLib {
    actual fun getInfo(): String {
        // TODO: Implement JS/WASM binding
        return "JS implementation pending"
    }

    actual fun setGravity(y: Float) {}
    actual fun setTimeScale(scale: Float) {}
    actual fun setPaused(paused: Boolean) {}
    actual fun resetSimulation() {}
}
