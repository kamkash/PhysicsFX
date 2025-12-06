package app.kamkash.physicsfx

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application

fun main() {
    System.err.println("DEBUG: Application starting (Rust Winit Mode)...")
    try {
        JvmWgpuGameLoop().runWinit()
    } catch (e: Throwable) {
        e.printStackTrace()
    }
    System.err.println("DEBUG: Application exited")
}