package app.kamkash.physicsfx

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.unit.IntSize

@Composable
actual fun WgpuNativeView(modifier: Modifier) {
    val gameLoop = remember { WasmWgpuGameLoop() }
    var canvasSize by remember { mutableStateOf(IntSize(800, 600)) }
    
    DisposableEffect(Unit) {
        onDispose {
            gameLoop.end()
        }
    }
    
    LaunchedEffect(canvasSize) {
        if (!gameLoop.isRunning() && canvasSize.width > 0 && canvasSize.height > 0) {
            gameLoop.start(null, canvasSize.width, canvasSize.height)
        } else if (gameLoop.isRunning()) {
            gameLoop.resize(canvasSize.width, canvasSize.height)
        }
    }
    
    Box(
        modifier = modifier
            .background(Color(0xFF2A2A2A))
            .onSizeChanged { size ->
                canvasSize = size
            },
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = "Wasm WebGPU Surface\n(Game Loop: ${if (gameLoop.isRunning()) "Running" else "Stopped"})",
            color = Color.White.copy(alpha = 0.7f)
        )
        
        if (gameLoop.isRunning()) {
            Text(
                text = "●",
                color = Color.Green,
                modifier = Modifier.align(Alignment.TopEnd)
            )
        }
    }
    
    // TODO: Implement Canvas element integration
}
