package app.kamkash.physicsfx

import android.view.Surface
import kotlinx.coroutines.*

class AndroidWgpuGameLoop : WgpuGameLoop {
    private var running = false
    private var gameLoopJob: Job? = null
    private val gameScope = CoroutineScope(Dispatchers.Default + SupervisorJob())

    private var lastFrameTime = System.nanoTime()
    private var frameCount = 0
    private var fpsTimer = 0.0

    // Native methods - surface is android.view.Surface object
    private external fun nativeInit(surface: Surface, width: Int, height: Int): Boolean
    private external fun nativeUpdate(deltaTime: Float)
    private external fun nativeRender()
    private external fun nativeResize(width: Int, height: Int)
    private external fun nativeShutdown()

    companion object {
        const val TARGET_FPS = 60
        const val FRAME_TIME_NS = 1_000_000_000L / TARGET_FPS

        init {
            System.loadLibrary("physics_core")
        }
    }

    override fun start(surfaceHandle: Any?, width: Int, height: Int) {
        if (running) {
            android.util.Log.d("AndroidWgpuGameLoop", "Game loop already running")
            return
        }

        android.util.Log.d("AndroidWgpuGameLoop", "Starting game loop: ${width}x${height}")

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

        // Initialize wgpu with the Surface
        val initialized = nativeInit(surface, width, height)
        if (!initialized) {
            android.util.Log.e("AndroidWgpuGameLoop", "Failed to initialize wgpu")
            return
        }

        running = true
        lastFrameTime = System.nanoTime()

        // Start game loop coroutine
        gameLoopJob =
                gameScope.launch {
                    while (running && isActive) {
                        val currentTime = System.nanoTime()
                        val deltaTimeNs = currentTime - lastFrameTime
                        lastFrameTime = currentTime

                        val deltaTime = deltaTimeNs / 1_000_000_000.0f

                        // Update
                        nativeUpdate(deltaTime)

                        // Render
                        nativeRender()

                        // FPS tracking
                        frameCount++
                        fpsTimer += deltaTime
                        if (fpsTimer >= 1.0) {
                            android.util.Log.d("AndroidWgpuGameLoop", "FPS: $frameCount")
                            frameCount = 0
                            fpsTimer = 0.0
                        }

                        // Frame pacing
                        val frameTimeElapsed = System.nanoTime() - currentTime
                        val sleepTime = FRAME_TIME_NS - frameTimeElapsed
                        if (sleepTime > 0) {
                            delay(sleepTime / 1_000_000) // Convert to ms
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
        if (!running) return
        android.util.Log.d("AndroidWgpuGameLoop", "Resizing to: ${width}x${height}")
        nativeResize(width, height)
    }

    override fun end() {
        if (!running) return

        android.util.Log.d("AndroidWgpuGameLoop", "Stopping game loop")
        running = false

        // Cancel game loop
        gameLoopJob?.cancel()
        runBlocking { gameLoopJob?.join() }

        // Cleanup wgpu
        nativeShutdown()
    }

    override fun isRunning(): Boolean = running
}
