package app.kamkash.physicsfx

import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { AndroidWgpuGameLoop() }

    // Track the last Surface we initialized wgpu against
    val currentSurface = remember { mutableStateOf<Surface?>(null) }

    DisposableEffect(Unit) { onDispose { gameLoop.end() } }

    AndroidView(
        modifier = modifier,
        factory = { context ->
            SurfaceView(context).apply {
                holder.addCallback(
                    object : SurfaceHolder.Callback {
                        override fun surfaceCreated(holder: SurfaceHolder) {
                            val w = width
                            val h = height
                            val surface = holder.surface
                            if (w > 0 && h > 0 && surface.isValid) {
                                currentSurface.value = surface
                                gameLoop.start(surface, w, h)
                            }
                        }

                        override fun surfaceChanged(
                            holder: SurfaceHolder,
                            format: Int,
                            width: Int,
                            height: Int
                        ) {
                            if (width <= 0 || height <= 0) return

                            val newSurface = holder.surface
                            val oldSurface = currentSurface.value

                            if (oldSurface == null || oldSurface != newSurface || !newSurface.isValid) {
                                // Underlying Surface (ANativeWindow) changed → full restart
                                if (gameLoop.isRunning()) {
                                    gameLoop.end()
                                }
                                if (newSurface.isValid) {
                                    currentSurface.value = newSurface
                                    gameLoop.start(newSurface, width, height)
                                }
                            } else {
                                // Same Surface, just a size change → resize only
                                if (gameLoop.isRunning()) {
                                    gameLoop.resize(width, height)
                                } else if (newSurface.isValid) {
                                    gameLoop.start(newSurface, width, height)
                                }
                            }
                        }

                        override fun surfaceDestroyed(holder: SurfaceHolder) {
                            currentSurface.value = null
                            gameLoop.end()
                        }
                    }
                )
            }
        }
    )
}
