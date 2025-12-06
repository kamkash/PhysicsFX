package app.kamkash.physicsfx

import kotlinx.browser.window
import org.w3c.dom.Window
import org.w3c.performance.Performance

import kotlin.js.Promise
import kotlin.js.JsAny

// Top-level JS interop for Date.now()
private fun dateNow(): Double = js("Date.now()")

// External declarations for WASM Rust bindings
// PhysicsCore moved to PhysicsCore.kt

class WasmWgpuGameLoop : WgpuGameLoop {
    private var running = false
    private var lastFrameTime = 0.0
    private var frameCount = 0
    private var fpsTimer = 0.0
    private var animationFrameId: Int = 0
    
    companion object {
        const val TARGET_FPS = 60
    }
    
    override fun start(surfaceHandle: Any?, width: Int, height: Int) {
        if (running) {
            println("Game loop already running")
            return
        }
        
        println("Starting Wasm game loop: ${width}x${height}")
        
        // surfaceHandle should be the canvas element ID (String)
        val canvasId: String = when (surfaceHandle) {
            is String -> surfaceHandle
            null -> {
                println("WARNING: No canvas ID provided, using default 'canvas'")
                "canvas"
            }
            else -> {
                println("WARNING: Unsupported surface handle type: ${surfaceHandle::class}")
                "canvas"
            }
        }
        
        // Initialize WASM module explicitly (required for --target web)
        PhysicsCore.init().then {
            println("WASM module initialized")
            
            // Initialize wgpu with canvas ID
            PhysicsCore.wasm_init(canvasId, width, height).then { result ->
                // result is JsAny (JS boolean true/false)
                // We convert via string to be safe without JsBoolean imports
                val initialized = result.toString() == "true"
                if (!initialized) {
                    println("Failed to initialize wgpu")
                } else {
                    println("WASM wgpu initialized successfully")
                    running = true
                    lastFrameTime = dateNow()
                    // Start render loop using requestAnimationFrame
                    startRenderLoop()
                }
                null
            }.catch { e ->
                println("Failed to initialize wgpu: ${e.toString()}")
                null
            }
            null
        }
    }
    
    private fun startRenderLoop() {
        if (!running) {
             println("Render loop stopped (running=false)")
             return
        }
        // println("Loop tick") // Uncomment for verbosity if needed, but for now we just want to know if it starts
        if (frameCount % 60 == 0) println("Loop active")
        
        
        val currentTime = dateNow()
        val deltaTime = ((currentTime - lastFrameTime) / 1000.0).toFloat()
        lastFrameTime = currentTime
        
        // Update
        try {
            PhysicsCore.wasm_update(deltaTime)
        } catch (e: Throwable) {
             println("Error in wasm_update: $e")
        }
        
        // Render
        try {
            PhysicsCore.wasm_render()
        } catch (e: Throwable) {
             println("Error in wasm_render: $e")
        }
        
        // FPS tracking
        frameCount++
        fpsTimer += deltaTime
        if (fpsTimer >= 1.0) {
            println("FPS: $frameCount")
            frameCount = 0
            fpsTimer = 0.0
        }
        
        // Schedule next frame
        animationFrameId = window.requestAnimationFrame { startRenderLoop() }
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
        try {
            PhysicsCore.wasm_resize(width, height)
        } catch (e: Exception) {
            // Ignore if not loaded yet
        }
    }
    
    override fun end() {
        if (!running) return
        
        println("Stopping Wasm game loop")
        running = false
        
        // Cancel pending animation frame
        window.cancelAnimationFrame(animationFrameId)
        
        // Cleanup wgpu
        try {
            PhysicsCore.wasm_shutdown()
        } catch (e: Exception) {
            // Ignore if not loaded yet
        }
    }
    
    override fun isRunning(): Boolean = running
}
