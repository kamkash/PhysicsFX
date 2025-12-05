package app.kamkash.physicsfx

/**
 * Game loop interface for managing the render/update cycle
 */
interface WgpuGameLoop {
    /**
     * Initialize wgpu and create surface
     * @param surfaceHandle Platform-specific surface handle (ANativeWindow, HWND, UIView, etc.)
     * @param width Surface width in pixels
     * @param height Surface height in pixels
     */
    fun start(surfaceHandle: Any?, width: Int, height: Int)
    
    /**
     * Update game logic
     * @param deltaTime Time since last update in seconds
     */
    fun update(deltaTime: Float)
    
    /**
     * Render frame
     */
    fun render()
    
    /**
     * Handle surface resize
     * @param width New width in pixels
     * @param height New height in pixels
     */
    fun resize(width: Int, height: Int)
    
    /**
     * Cleanup and shutdown wgpu
     */
    fun end()
    
    /**
     * Check if game loop is running
     */
    fun isRunning(): Boolean
}
