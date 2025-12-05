package app.kamkash.physicsfx

import kotlinx.cinterop.*
import physics_core.*

@OptIn(ExperimentalForeignApi::class)
actual object NativeLib {
    actual fun getInfo(): String {
        val cString = physics_core_get_info()
        val result = cString?.toKString() ?: "Error getting info"
        physics_core_free_string(cString)
        return result
    }
}
