package app.kamkash.physicsfx

expect object NativeLib {
    fun getInfo(): String
    fun setGravity(y: Float)
    fun setTimeScale(scale: Float)
    fun setPaused(paused: Boolean)
    fun resetSimulation()
}
