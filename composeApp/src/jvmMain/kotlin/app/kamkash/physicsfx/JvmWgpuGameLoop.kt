package app.kamkash.physicsfx

import kotlinx.coroutines.*

class JvmWgpuGameLoop : WgpuGameLoop {
    private var running = false
    private var gameLoopJob: Job? = null
    // Use Dispatchers.Main (Swing EDT) for rendering as required by Metal/wgpu
    // private val gameScope = CoroutineScope(Dispatchers.Main + SupervisorJob())
    // In JvmWgpuGameLoop.kt
    private val gameScope by lazy { CoroutineScope(Dispatchers.Main + SupervisorJob()) }

    private var lastFrameTime = System.nanoTime()
    private var frameCount = 0
    private var fpsTimer = 0.0

    // Native methods - surfaceHandle is a raw pointer (0 for JVM since we don't have easy access)
    private external fun nativeInit(surfaceHandle: Long, width: Int, height: Int): Boolean
    private external fun nativeUpdate(deltaTime: Float)
    private external fun nativeRender()
    private external fun nativeResize(width: Int, height: Int)
    private external fun nativeShutdown()
    private external fun nativeStartWinitApp()

    companion object {
        const val TARGET_FPS = 60
        const val FRAME_TIME_NS = 1_000_000_000L / TARGET_FPS

        init {
            System.loadLibrary("physics_core")
        }
    }

    override fun start(surfaceHandle: Any?, width: Int, height: Int) {
        if (running) {
            println("Game loop already running")
            return
        }

        println(
                "Starting JVM game loop: ${width}x${height} on thread: ${Thread.currentThread().name}"
        )

        // For JVM desktop, we don't have easy access to native window handles from Compose
        // Pass 0 to indicate no surface - wgpu will operate in headless mode or skip rendering
        val surfacePtr: Long =
                when (surfaceHandle) {
                    is Long -> surfaceHandle
                    null -> {
                        println(
                                "WARNING: No surface handle provided for JVM. Surface-based rendering not available."
                        )
                        0L
                    }
                    else -> {
                        println(
                                "WARNING: Unsupported surface handle type: ${surfaceHandle::class}. Using null."
                        )
                        0L
                    }
                }

        // Initialize wgpu
        println(
                "DEBUG: Calling nativeInit with surfacePtr=0x${surfacePtr.toString(16)}, size=${width}x${height}"
        )
        val initialized = nativeInit(surfacePtr, width, height)
        println("DEBUG: nativeInit returned: $initialized")
        if (!initialized) {
            println("ERROR: Failed to initialize wgpu")
            return
        }

        running = true
        lastFrameTime = System.nanoTime()

        // Start game loop coroutine
        gameLoopJob =
                gameScope.launch {
                    println(
                            "DEBUG: Game loop coroutine started on thread: ${Thread.currentThread().name}"
                    )
                    while (running && isActive) {
                        val currentTime = System.nanoTime()
                        val deltaTimeNs = currentTime - lastFrameTime
                        lastFrameTime = currentTime

                        val deltaTime = deltaTimeNs / 1_000_000_000.0f

                        // Update
                        // println("DEBUG: Calling nativeUpdate")
                        nativeUpdate(deltaTime)

                        // Render
                        // println("DEBUG: Calling nativeRender")
                        nativeRender()

                        // FPS tracking
                        frameCount++
                        fpsTimer += deltaTime
                        if (fpsTimer >= 1.0) {
                            println("FPS: $frameCount")
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
                    println("DEBUG: Game loop coroutine exited")
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
        println("DEBUG: Resizing to: ${width}x${height}")
        nativeResize(width, height)
    }

    override fun end() {
        if (!running) return

        println("DEBUG: Stopping JVM game loop")
        running = false

        // Cancel game loop
        gameLoopJob?.cancel()
        runBlocking { gameLoopJob?.join() }

        // Cleanup wgpu
        println("DEBUG: Calling nativeShutdown")
        nativeShutdown()
    }

    override fun isRunning(): Boolean = running

    fun runWinit() {
        println("Calling nativeStartWinitApp...")
        nativeStartWinitApp()
    }
}
