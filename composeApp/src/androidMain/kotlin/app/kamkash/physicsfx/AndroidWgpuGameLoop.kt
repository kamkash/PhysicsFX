package app.kamkash.physicsfx

import android.view.Surface
import kotlinx.coroutines.*

object AndroidWgpuConfig {
    // Set to true only when you explicitly want to exercise the native wgpu path
    // on a device you trust. Default is false for stability.
    const val ENABLE_NATIVE_WGPU: Boolean = true
}

class AndroidWgpuGameLoop : WgpuGameLoop {
    private var running = false
    private var gameLoopJob: Job? = null
    // Run the game loop on the main thread to match surface / device ownership
    private val gameScope = CoroutineScope(Dispatchers.Main.immediate + SupervisorJob())

    private var lastFrameTime = System.nanoTime()
    private var frameCount = 0
    private var fpsTimer = 0.0
    @Volatile private var surfaceReady = false

    // Native methods - surface is android.view.Surface object
    private external fun nativeInit(surface: Surface, width: Int, height: Int): Boolean
    private external fun nativeUpdate(deltaTime: Float)
    private external fun nativeRender()
    private external fun nativeResize(width: Int, height: Int)
    private external fun nativeShutdown()

    companion object {
        const val TARGET_FPS = 60
        const val FRAME_TIME_NS = 1_000_000_000L / TARGET_FPS
        // For sparse rendering test, render at most once per second when enabled
        const val RENDER_INTERVAL_SEC = 1.0f

        init {
            System.loadLibrary("physics_core")
        }
    }

    override fun start(surfaceHandle: Any?, width: Int, height: Int) {
        if (running) {
            android.util.Log.d("AndroidWgpuGameLoop", "Game loop already running")
            return
        }

        if (width <= 0 || height <= 0) {
            android.util.Log.w(
                    "AndroidWgpuGameLoop",
                    "Ignoring start with non-positive size: ${width}x${height}"
            )
            return
        }

        if (!AndroidWgpuConfig.ENABLE_NATIVE_WGPU) {
            // SAFE DEFAULT: no native wgpu, just a Kotlin timing loop
            android.util.Log.d(
                    "AndroidWgpuGameLoop",
                    "Starting game loop in SAFE mode (no native wgpu): ${width}x${height}"
            )
            running = true
            surfaceReady = false
            lastFrameTime = System.nanoTime()

            gameLoopJob =
                    gameScope.launch {
                        while (running && isActive) {
                            val currentTime = System.nanoTime()
                            val deltaTimeNs = currentTime - lastFrameTime
                            lastFrameTime = currentTime

                            val deltaTime = deltaTimeNs / 1_000_000_000.0f

                            // FPS tracking only
                            frameCount++
                            fpsTimer += deltaTime
                            if (fpsTimer >= 1.0f) {
                                android.util.Log.d(
                                        "AndroidWgpuGameLoop",
                                        "(SAFE no-native) FPS ticks: $frameCount"
                                )
                                frameCount = 0
                                fpsTimer = 0.0
                            }

                            val frameTimeElapsed = System.nanoTime() - currentTime
                            val sleepTime = FRAME_TIME_NS - frameTimeElapsed
                            if (sleepTime > 0) {
                                delay(sleepTime / 1_000_000)
                            }
                        }
                    }
            return
        }

        // EXPERIMENTAL: native wgpu path (use only on trusted devices)
        android.util.Log.d(
                "AndroidWgpuGameLoop",
                "Starting game loop (native wgpu ENABLED): ${width}x${height}"
        )

        // surfaceHandle must be an android.view.Surface
        val surface: Surface =
                when (surfaceHandle) {
                    is Surface -> surfaceHandle
                    null -> {
                        android.util.Log.e("AndroidWgpuGameLoop", "No surface provided")
                        return
                    }
                    else -> {
                        android.util.Log.e(
                                "AndroidWgpuGameLoop",
                                "Invalid surface type: ${surfaceHandle::class}"
                        )
                        return
                    }
                }

        val initialized = nativeInit(surface, width, height)
        if (!initialized) {
            android.util.Log.e("AndroidWgpuGameLoop", "Failed to initialize wgpu")
            return
        }

        running = true
        surfaceReady = false
        lastFrameTime = System.nanoTime()

        gameLoopJob =
                gameScope.launch {
                    var timeSinceLastRender = 0.0f

                    while (running && isActive) {
                        val currentTime = System.nanoTime()
                        val deltaTimeNs = currentTime - lastFrameTime
                        lastFrameTime = currentTime

                        val deltaTime = deltaTimeNs / 1_000_000_000.0f

                        timeSinceLastRender += deltaTime
                        if (timeSinceLastRender >= RENDER_INTERVAL_SEC) {
                            android.util.Log.d(
                                    "AndroidWgpuGameLoop",
                                    "Calling nativeUpdate/render after ${timeSinceLastRender}s"
                            )
                            try {
                                nativeUpdate(timeSinceLastRender)
                                nativeRender()
                            } catch (t: Throwable) {
                                android.util.Log.e(
                                        "AndroidWgpuGameLoop",
                                        "Error in nativeUpdate/render: ${t.message}",
                                        t
                                )
                                running = false
                                break
                            }
                            timeSinceLastRender = 0.0f
                        }

                        frameCount++
                        fpsTimer += deltaTime
                        if (fpsTimer >= 1.0f) {
                            android.util.Log.d(
                                    "AndroidWgpuGameLoop",
                                    "(native sparse-render) FPS ticks: $frameCount"
                            )
                            frameCount = 0
                            fpsTimer = 0.0
                        }

                        val frameTimeElapsed = System.nanoTime() - currentTime
                        val sleepTime = FRAME_TIME_NS - frameTimeElapsed
                        if (sleepTime > 0) {
                            delay(sleepTime / 1_000_000)
                        }
                    }
                }
    }

    override fun update(deltaTime: Float) {
        // Update is called internally in the loop
    }

    override fun render() {
        // Render is called internally in the loop
    }

    override fun resize(width: Int, height: Int) {
        if (!running || width <= 0 || height <= 0) return

        if (!AndroidWgpuConfig.ENABLE_NATIVE_WGPU) {
            android.util.Log.d(
                    "AndroidWgpuGameLoop",
                    "Ignoring resize in SAFE mode: ${width}x${height}"
            )
            return
        }

        android.util.Log.d(
                "AndroidWgpuGameLoop",
                "Ignoring resize in native sparse-render mode: ${width}x${height}"
        )
        // If you later want to exercise nativeResize, wire it here.
    }

    override fun end() {
        if (!running) return

        android.util.Log.d("AndroidWgpuGameLoop", "Stopping game loop")
        running = false
        surfaceReady = false

        gameLoopJob?.cancel()
        gameLoopJob = null

        if (AndroidWgpuConfig.ENABLE_NATIVE_WGPU) {
            nativeShutdown()
        }
    }

    override fun isRunning(): Boolean = running
}
