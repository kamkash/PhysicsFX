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
    val gameLoop = remember { AndroidWgpuGameLoop() }
    var surfaceSize by remember { mutableStateOf(IntSize(800, 600)) }
    
    DisposableEffect(Unit) {
        onDispose {
            gameLoop.end()
        }
    }
    
    LaunchedEffect(surfaceSize) {
        if (!gameLoop.isRunning() && surfaceSize.width > 0 && surfaceSize.height > 0) {
            gameLoop.start(null, surfaceSize.width, surfaceSize.height)
        } else if (gameLoop.isRunning()) {
            gameLoop.resize(surfaceSize.width, surfaceSize.height)
        }
    }
    
    Box(
        modifier = modifier
            .background(Color(0xFF2A2A2A))
            .onSizeChanged { size ->
                surfaceSize = size
            },
        contentAlignment = Alignment.Center
    ) {
        Text(
            text = "Android WebGPU Surface\n(Game Loop: ${if (gameLoop.isRunning()) "Running" else "Stopped"})",
            color = Color.White.copy(alpha = 0.7f)
        )
        
        // Status indicator
        if (gameLoop.isRunning()) {
            Text(
                text = "●",
                color = Color.Green,
                modifier = Modifier.align(Alignment.TopEnd)
            )
        }
    }
    
    // TODO: Replace with AndroidView + SurfaceView
    // AndroidView(
    //     factory = { context ->
    //         SurfaceView(context).apply {
    //             holder.addCallback(...)
    //         }
    //     }
    // )
}
