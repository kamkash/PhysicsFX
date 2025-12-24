package app.kamkash.physicsfx

import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.input.key.*
import androidx.compose.ui.input.pointer.PointerInputChange
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.viewinterop.AndroidView

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { AndroidWgpuGameLoop() }

    // Track the last Surface we initialized wgpu against
    val currentSurface = remember { mutableStateOf<Surface?>(null) }

    DisposableEffect(Unit) { onDispose { gameLoop.end() } }

    AndroidView(
            modifier =
                    modifier
                            .pointerInput(Unit) {
                                detectDragGestures(
                                        onDragStart = { offset: Offset ->
                                            NativeLib.onPointerEvent(0, offset.x, offset.y, 0)
                                        },
                                        onDrag = { change: PointerInputChange, _ ->
                                            NativeLib.onPointerEvent(
                                                    1,
                                                    change.position.x,
                                                    change.position.y,
                                                    0
                                            )
                                        },
                                        onDragEnd = { NativeLib.onPointerEvent(2, -1f, -1f, 0) },
                                        onDragCancel = { NativeLib.onPointerEvent(2, -1f, -1f, 0) }
                                )
                            }
                            .onKeyEvent { keyEvent ->
                                val et = if (keyEvent.type == KeyEventType.KeyDown) 0 else 1
                                NativeLib.onKeyEvent(et, keyEvent.nativeKeyEvent.keyCode)
                                false // allow other handlers
                            },
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

                                    if (oldSurface == null ||
                                                    oldSurface != newSurface ||
                                                    !newSurface.isValid
                                    ) {
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
