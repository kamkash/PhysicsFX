package app.kamkash.physicsfx

import java.awt.Frame
import javax.swing.JFrame
import kotlin.system.exitProcess

fun main() {
    System.err.println("DEBUG: Application starting (Rust Winit Mode)...")
    try {
        requestFocus()
        JvmWgpuGameLoop().runWinit()
    } catch (e: Throwable) {
        e.printStackTrace()
    }
    System.err.println("DEBUG: Application exited")
    exitProcess(0)
}

private fun requestFocus() {
    // A more forceful trick to bring the application to the front on Windows.
    // We create a temporary, invisible frame, make it active, and then dispose of it.
    // This can help transfer focus to the application's main window when it appears.
    val frame = JFrame()
    frame.isUndecorated = true
    frame.isAlwaysOnTop = true
    frame.pack()
    frame.setLocationRelativeTo(null)
    frame.isVisible = true
    frame.toFront()
    frame.dispose()
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
