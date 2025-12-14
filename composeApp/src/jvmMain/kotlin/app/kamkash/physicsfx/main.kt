package app.kamkash.physicsfx

import kotlin.system.exitProcess

fun main() {
    System.err.println("DEBUG: Application starting (Rust Winit Mode)...")
    try {
        JvmWgpuGameLoop().runWinit()
    } catch (e: Throwable) {
        e.printStackTrace()
    }
    System.err.println("DEBUG: Application exited")
    exitProcess(0)
}

// import androidx.compose.material3.MaterialTheme
// import androidx.compose.material3.Text
// import androidx.compose.ui.window.Window
// import androidx.compose.ui.window.application

//  ***  ⚠️  "-XstartOnFirstThread",   build.gradle.kts  ***
// fun main() = application {
//     Window(
//             onCloseRequest = ::exitApplication,
//             title = "PhysicsFX",
//     ) {
//         // App()
//         Text(
//                 text = "JVM Main",
//                 style = MaterialTheme.typography.labelLarge,
//                 color = MaterialTheme.colorScheme.onSurfaceVariant
//         )
//     }
// }
