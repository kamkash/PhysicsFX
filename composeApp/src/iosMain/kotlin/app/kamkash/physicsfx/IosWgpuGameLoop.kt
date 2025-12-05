package app.kamkash.physicsfx

import kotlinx.cinterop.*
import physics_core.*
import kotlinx.coroutines.*
import platform.posix.usleep
import platform.Foundation.*

@OptIn(ExperimentalForeignApi::class)
class IosWgpuGameLoop : WgpuGameLoop {
    private var running = false
    private var gameLoopJob: Job? = null
    private val gameScope = CoroutineScope(Dispatchers.Default + SupervisorJob())
    
    private var lastFrameTime = platform.Foundation.NSDate().timeIntervalSince1970
    private var frameCount = 0
    private var fpsTimer = 0.0
    
    companion object {
        const val TARGET_FPS = 60
        const val FRAME_TIME_SECONDS = 1.0 / TARGET_FPS
    }
    
    override fun start(surfaceHandle: Any?, width: Int, height: Int) {
        if (running) {
            println("Game loop already running")
            return
        }
        
        println("Starting iOS game loop: ${width}x${height}")
        
        // Initialize wgpu via C interop
        val initialized = wgpu_init(width, height)
        if (!initialized) {
            println("Failed to initialize wgpu")
            return
        }
        
        running = true
        lastFrameTime = platform.Foundation.NSDate().timeIntervalSince1970
        
        // Start game loop coroutine
        gameLoopJob = gameScope.launch {
            while (running && isActive) {
                val currentTime = platform.Foundation.NSDate().timeIntervalSince1970
                val deltaTime = (currentTime - lastFrameTime).toFloat()
                lastFrameTime = currentTime
                
                // Update
                wgpu_update(deltaTime)
                
                // Render
                wgpu_render()
                
                // FPS tracking
                frameCount++
                fpsTimer += deltaTime
                if (fpsTimer >= 1.0) {
                    println("FPS: $frameCount")
                    frameCount = 0
                    fpsTimer = 0.0
                }
                
                // Frame pacing
                val frameTimeElapsed = platform.Foundation.NSDate().timeIntervalSince1970 - currentTime
                val sleepTime = FRAME_TIME_SECONDS - frameTimeElapsed
                if (sleepTime > 0) {
                    delay((sleepTime * 1000).toLong()) // Convert to ms
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
        println("Resizing to: ${width}x${height}")
        wgpu_resize(width, height)
    }
    
    override fun end() {
        if (!running) return
        
        println("Stopping iOS game loop")
        running = false
        
        // Cancel game loop
        gameLoopJob?.cancel()
        runBlocking {
            gameLoopJob?.join()
        }
        
        // Cleanup wgpu
        wgpu_shutdown()
    }
    
    override fun isRunning(): Boolean = running
}
