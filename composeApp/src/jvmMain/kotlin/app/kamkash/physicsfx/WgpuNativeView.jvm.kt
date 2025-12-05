package app.kamkash.physicsfx

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.awt.SwingPanel
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.unit.IntSize
import java.awt.Canvas
import java.awt.Dimension

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { JvmWgpuGameLoop() }
    var canvasSize by remember { mutableStateOf(IntSize(800, 600)) }
    
    DisposableEffect(Unit) {
        onDispose {
            gameLoop.end()
        }
    }
    
    Box(
        modifier = modifier
            .background(Color(0xFF2A2A2A))
            .onSizeChanged { size ->
                canvasSize = size
                if (gameLoop.isRunning()) {
                    gameLoop.resize(size.width, size.height)
                }
            }
    ) {
        SwingPanel(
            modifier = Modifier.fillMaxSize(),
            factory = {
                Canvas().apply {
                    preferredSize = Dimension(canvasSize.width, canvasSize.height)
                }
            },
            update = { canvas ->
                if (canvas.width != canvasSize.width || canvas.height != canvasSize.height) {
                    canvas.setSize(canvasSize.width, canvasSize.height)
                }
            }
        )
        
        // Start game loop once size is known
        LaunchedEffect(canvasSize) {
            if (!gameLoop.isRunning() && canvasSize.width > 0 && canvasSize.height > 0) {
                gameLoop.start(null, canvasSize.width, canvasSize.height)
            }
        }
        
        // Status overlay
        if (gameLoop.isRunning()) {
            Text(
                text = "Game Loop Active",
                color = Color.Green.copy(alpha = 0.7f),
                modifier = Modifier
                    .align(Alignment.TopStart)
            )
        }
    }
}
