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

    actual fun setGravity(y: Float) {
        physics_core_set_gravity(y)
    }

    actual fun setTimeScale(scale: Float) {
        physics_core_set_time_scale(scale)
    }

    actual fun setPaused(paused: Boolean) {
        physics_core_set_paused(paused)
    }

    actual fun resetSimulation() {
        physics_core_reset_simulation()
    }
}
