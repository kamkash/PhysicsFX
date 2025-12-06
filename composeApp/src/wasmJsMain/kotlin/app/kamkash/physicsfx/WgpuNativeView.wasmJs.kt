package app.kamkash.physicsfx

import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.foundation.layout.Box
import androidx.compose.ui.layout.onSizeChanged
import kotlinx.browser.document
import kotlinx.browser.window
import org.w3c.dom.HTMLCanvasElement

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { WasmWgpuGameLoop() }
    val canvasId = "wgpu-canvas-overlay" // RENAMED to ensure uniqueness
    
    DisposableEffect(Unit) {
        onDispose {
            gameLoop.end()
        }
    }
    
    DisposableEffect(Unit) {
        val canvas = document.createElement("canvas") as HTMLCanvasElement
        canvas.id = canvasId
        canvas.style.position = "fixed" 
        canvas.style.top = "64px" // Below the toolbar
        canvas.style.left = "0px"
        canvas.style.zIndex = "100" // Above Compose Shadow DOM but reasonable
        // Width/height set dynamically in onSizeChanged
        
        // Append to documentElement (html) instead of body to bypass Shadow DOM
        document.documentElement?.appendChild(canvas)
        println("DEBUG: Appended canvas $canvasId to documentElement")
        
        onDispose {
            println("DEBUG: Removing canvas $canvasId")
            document.documentElement?.removeChild(canvas)
        }
    }

    Box(
        modifier = modifier.onSizeChanged { size ->
            val canvas = document.getElementById(canvasId) as? HTMLCanvasElement
            if (canvas != null && size.width > 0 && size.height > 0) {
                // Clamp to WebGL2 max texture dimension (matches Rust-side clamping)
                val maxDim = 2048
                val clampedWidth = minOf(size.width, maxDim)
                val clampedHeight = minOf(size.height, maxDim)
                
                // Update canvas CSS dimensions to match clamped size
                canvas.style.width = "${clampedWidth}px"
                canvas.style.height = "${clampedHeight}px"
                
                // Update internal resolution
                canvas.width = clampedWidth
                canvas.height = clampedHeight
                
                if (gameLoop.isRunning()) {
                    gameLoop.resize(clampedWidth, clampedHeight)
                } else {
                    gameLoop.start(canvasId, clampedWidth, clampedHeight)
                }
            }
        }
    )
}
