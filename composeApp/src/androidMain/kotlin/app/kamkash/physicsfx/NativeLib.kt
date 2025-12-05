package app.kamkash.physicsfx

actual object NativeLib {
    init {
        System.loadLibrary("physics_core")
    }
    external actual fun getInfo(): String
}
