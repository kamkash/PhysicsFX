package app.kamkash.physicsfx

expect object NativeLib {
    fun getInfo(): String
    fun setGravity(y: Float)
    fun setTimeScale(scale: Float)
    fun setPaused(paused: Boolean)
    fun resetSimulation()
    fun onPointerEvent(eventType: Int, x: Float, y: Float, button: Int)
    fun onKeyEvent(eventType: Int, keyCode: Int)
}
