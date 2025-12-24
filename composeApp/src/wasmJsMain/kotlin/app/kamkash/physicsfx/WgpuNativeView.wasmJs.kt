package app.kamkash.physicsfx

import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.onSizeChanged
import kotlinx.browser.document
import org.w3c.dom.HTMLCanvasElement

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { WasmWgpuGameLoop() }
    val canvasId = "wgpu-canvas-overlay" // RENAMED to ensure uniqueness

    DisposableEffect(Unit) { onDispose { gameLoop.end() } }

    DisposableEffect(Unit) {
        val canvas = document.createElement("canvas") as HTMLCanvasElement
        canvas.id = canvasId
        canvas.style.position = "fixed"
        canvas.style.top = "64px" // Below the toolbar
        canvas.style.left = "0px"
        canvas.style.width = "400px"
        canvas.style.height = "400px"
        canvas.width = 400
        canvas.height = 400

        canvas.style.zIndex = "100" // Above Compose Shadow DOM but reasonable
        // Width/height set dynamically in onSizeChanged

        // Append to documentElement (html) instead of body to bypass Shadow DOM
        document.documentElement?.appendChild(canvas)
        println("DEBUG: Appended canvas $canvasId to documentElement")

        // Mouse Events
        canvas.addEventListener(
                "mousedown",
                { event ->
                    val mouseEvent = event as org.w3c.dom.events.MouseEvent
                    NativeLib.onPointerEvent(
                            0,
                            mouseEvent.offsetX.toFloat(),
                            mouseEvent.offsetY.toFloat(),
                            mouseEvent.button.toInt()
                    )
                }
        )
        canvas.addEventListener(
                "mousemove",
                { event ->
                    val mouseEvent = event as org.w3c.dom.events.MouseEvent
                    NativeLib.onPointerEvent(
                            1,
                            mouseEvent.offsetX.toFloat(),
                            mouseEvent.offsetY.toFloat(),
                            mouseEvent.button.toInt()
                    )
                }
        )
        canvas.addEventListener(
                "mouseup",
                { event ->
                    val mouseEvent = event as org.w3c.dom.events.MouseEvent
                    NativeLib.onPointerEvent(
                            2,
                            mouseEvent.offsetX.toFloat(),
                            mouseEvent.offsetY.toFloat(),
                            mouseEvent.button.toInt()
                    )
                }
        )
        // Keyboard Events
        document.addEventListener(
                "keydown",
                { event ->
                    val keyEvent = event as org.w3c.dom.events.KeyboardEvent
                    NativeLib.onKeyEvent(0, keyEvent.keyCode)
                }
        )
        document.addEventListener(
                "keyup",
                { event ->
                    val keyEvent = event as org.w3c.dom.events.KeyboardEvent
                    NativeLib.onKeyEvent(1, keyEvent.keyCode)
                }
        )

        onDispose {
            println("DEBUG: Removing canvas $canvasId")
            document.documentElement?.removeChild(canvas)
        }
    }

    Box(
            modifier =
                    modifier.onSizeChanged { size ->
                        println("DEBUG: Canvas size changed to ${size.width}x${size.height}")
                        val canvas = document.getElementById(canvasId) as? HTMLCanvasElement
                        if (canvas != null && size.width > 0 && size.height > 0) {
                            // Clamp to WebGL2 max texture dimension (matches Rust-side clamping)
                            val maxDim = 2048
                            val clampedWidth = minOf(size.width, maxDim)
                            val clampedHeight = minOf(size.height, maxDim)

                            val width = 400
                            val height = 400

                            // Update canvas CSS dimensions to match clamped size
                            // canvas.style.width = "${clampedWidth}px"
                            // canvas.style.height = "${clampedHeight}px"

                            // // Update internal resolution
                            // canvas.width = clampedWidth
                            // canvas.height = clampedHeight

                            if (gameLoop.isRunning()) {
                                // gameLoop.resize(clampedWidth, clampedHeight)
                                gameLoop.resize(width, height)
                            } else {
                                // gameLoop.start(canvasId, clampedWidth, clampedHeight)
                                gameLoop.start(canvasId, width, height)
                            }
                        }
                    }
    )
}
