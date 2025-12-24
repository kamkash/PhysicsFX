package app.kamkash.physicsfx

actual object NativeLib {
    init {
        System.loadLibrary("physics_core")
    }
    external actual fun getInfo(): String
    external actual fun setGravity(y: Float)
    external actual fun setTimeScale(scale: Float)
    external actual fun setPaused(paused: Boolean)
    external actual fun resetSimulation()
    external actual fun onPointerEvent(eventType: Int, x: Float, y: Float, button: Int)
    external actual fun onKeyEvent(eventType: Int, keyCode: Int)
}
