package app.kamkash.physicsfx

import kotlinx.browser.window
import org.w3c.dom.Window
import org.w3c.performance.Performance

// Top-level JS interop for Date.now()
private fun dateNow(): Double = js("Date.now()")

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
        
        // TODO: Initialize wgpu via wasm-bindgen
        // val initialized = wasm_init(width.toUInt(), height.toUInt())
        val initialized = true
        
        if (!initialized) {
            println("Failed to initialize wgpu")
            return
        }
        
        running = true
        lastFrameTime = dateNow()
        
        // Start render loop using requestAnimationFrame
        startRenderLoop()
    }
    
    private fun startRenderLoop() {
        if (!running) return
        
        val currentTime = dateNow()
        val deltaTime = ((currentTime - lastFrameTime) / 1000.0).toFloat()
        lastFrameTime = currentTime
        
        // Update
        // wasm_update(deltaTime)
        
        // Render
        // wasm_render()
        
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
        // wasm_resize(width.toUInt(), height.toUInt())
    }
    
    override fun end() {
        if (!running) return
        
        println("Stopping Wasm game loop")
        running = false
        
        // Cancel pending animation frame
        window.cancelAnimationFrame(animationFrameId)
        
        // Cleanup wgpu
        // wasm_shutdown()
    }
    
    override fun isRunning(): Boolean = running
}
