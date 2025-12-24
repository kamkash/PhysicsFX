package app.kamkash.physicsfx

import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.interop.UIKitView
import kotlinx.cinterop.*
import platform.CoreGraphics.CGRect
import platform.CoreGraphics.CGRectMake
import platform.Foundation.NSCoder
import platform.QuartzCore.CAMetalLayer
import platform.UIKit.*

@OptIn(ExperimentalForeignApi::class)
class MetalView : UIView {
    @OverrideInit constructor(frame: CValue<CGRect>) : super(frame = frame)
    @OverrideInit constructor(coder: NSCoder) : super(coder = coder)

    private var loop: WgpuGameLoop? = null

    fun setGameLoop(gameLoop: WgpuGameLoop) {
        this.loop = gameLoop
    }

    override fun layoutSubviews() {
        super.layoutSubviews()

        val scale = contentScaleFactor
        val width = (frame.useContents { size.width } * scale).toInt()
        val height = (frame.useContents { size.height } * scale).toInt()

        if (width > 0 && height > 0) {
            loop?.resize(width, height)
        }
    }

    override fun touchesBegan(touches: Set<*>, withEvent: UIEvent?) {
        super.touchesBegan(touches, withEvent)
        handleTouches(touches, 0)
    }

    override fun touchesMoved(touches: Set<*>, withEvent: UIEvent?) {
        super.touchesMoved(touches, withEvent)
        handleTouches(touches, 1)
    }

    override fun touchesEnded(touches: Set<*>, withEvent: UIEvent?) {
        super.touchesEnded(touches, withEvent)
        handleTouches(touches, 2)
    }

    override fun touchesCancelled(touches: Set<*>, withEvent: UIEvent?) {
        super.touchesCancelled(touches, withEvent)
        handleTouches(touches, 2)
    }

    private fun handleTouches(touches: Set<*>, eventType: Int) {
        val touch = touches.firstOrNull() as? UITouch ?: return
        val location = touch.locationInView(this)
        val scale = contentScaleFactor
        NativeLib.onPointerEvent(
                eventType,
                (location.useContents { x } * scale).toFloat(),
                (location.useContents { y } * scale).toFloat(),
                0
        )
    }

    companion object : UIViewMeta() {
        override fun layerClass() = CAMetalLayer.`class`()!!
    }
}

@OptIn(ExperimentalForeignApi::class)
@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { IosWgpuGameLoop() }

    DisposableEffect(Unit) { onDispose { gameLoop.end() } }

    UIKitView(
            factory = {
                // Use custom MetalView to have full control over the layer
                val view = MetalView(frame = CGRectMake(0.0, 0.0, 1.0, 1.0))
                view.setGameLoop(gameLoop)

                // Configure view properties
                view.contentScaleFactor = UIScreen.mainScreen.scale
                view.setUserInteractionEnabled(true)
                view.setMultipleTouchEnabled(true)
                view.opaque = true
                view.backgroundColor = UIColor.blackColor

                // Configure Metal Layer explicitly
                val layer = view.layer as CAMetalLayer
                layer.contentsScale = view.contentScaleFactor
                // layer.presentsWithTransaction = false // Low latency
                // layer.framebufferOnly = true

                // Calculate initial size
                val scale = view.contentScaleFactor
                val width = (view.frame.useContents { size.width } * scale).toInt()
                val height = (view.frame.useContents { size.height } * scale).toInt()

                println(
                        "WgpuNativeView Factory: frame=${view.frame.useContents{size.width} }x${view.frame.useContents{size.height}}, scale=$scale, pixels=${width}x${height}"
                )

                if (width > 0 && height > 0) {
                    println("Starting game loop with view: $width x $height")
                    // Pass the raw pointer to the view correctly
                    val viewPtr = interpretCPointer<CPointed>(view.objcPtr())
                    gameLoop.start(viewPtr, width, height)
                }

                view
            },
            modifier = modifier,
            update = { view: MetalView ->
                // Layout updates are handled in MetalView.layoutSubviews
                // We just check if we need to start the loop if it wasn't started (e.g. init was
                // 0x0)

                val scale = view.contentScaleFactor
                val width = (view.frame.useContents { size.width } * scale).toInt()
                val height = (view.frame.useContents { size.height } * scale).toInt()

                if (!gameLoop.isRunning() && width > 0 && height > 0) {
                    val viewPtr = interpretCPointer<CPointed>(view.objcPtr())
                    gameLoop.start(viewPtr, width, height)
                }
            },
            onRelease = { gameLoop.end() }
    )
}
