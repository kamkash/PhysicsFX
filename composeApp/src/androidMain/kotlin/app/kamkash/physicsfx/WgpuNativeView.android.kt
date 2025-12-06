package app.kamkash.physicsfx

import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { AndroidWgpuGameLoop() }

    DisposableEffect(Unit) { onDispose { gameLoop.end() } }

    AndroidView(
            modifier = modifier,
            factory = { context ->
                SurfaceView(context).apply {
                    holder.addCallback(
                            object : SurfaceHolder.Callback {
                                override fun surfaceCreated(holder: SurfaceHolder) {
                                    // Start loop with the surface
                                    val width = width
                                    val height = height
                                    if (width > 0 && height > 0) {
                                        gameLoop.start(holder.surface, width, height)
                                        // Set scale factor - REMOVED
                                    }
                                }

                                override fun surfaceChanged(
                                        holder: SurfaceHolder,
                                        format: Int,
                                        width: Int,
                                        height: Int
                                ) {
                                    if (gameLoop.isRunning()) {
                                        gameLoop.resize(width, height)
                                    } else if (width > 0 && height > 0) {
                                        gameLoop.start(holder.surface, width, height)
                                    }
                                }

                                override fun surfaceDestroyed(holder: SurfaceHolder) {
                                    gameLoop.end()
                                }
                            }
                    )
                }
            }
    )
}
