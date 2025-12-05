package app.kamkash.physicsfx

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import org.jetbrains.compose.ui.tooling.preview.Preview

@OptIn(ExperimentalMaterial3Api::class)
@Composable
@Preview
fun App() {
    MaterialTheme {
        Scaffold(
            topBar = {
                MainToolbar()
            }
        ) { paddingValues ->
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(paddingValues)
            ) {
                // WebGPU Rendering Surface
                WgpuRenderSurface(
                    modifier = Modifier
                        .fillMaxWidth()
                        .weight(1f)
                )
                
                // Info panel
                InfoPanel()
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainToolbar() {
    var showMenu by remember { mutableStateOf(false) }
    
    TopAppBar(
        title = { Text("PhysicsFX") },
        colors = TopAppBarDefaults.topAppBarColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer,
            titleContentColor = MaterialTheme.colorScheme.onPrimaryContainer,
        ),
        actions = {
            TextButton(onClick = { /* Play/Pause simulation */ }) {
                Text("▶")
            }
            TextButton(onClick = { /* Reset simulation */ }) {
                Text("⟳")
            }
            TextButton(onClick = { /* Settings */ }) {
                Text("⚙")
            }
            TextButton(onClick = { showMenu = !showMenu }) {
                Text("⋮")
            }
            DropdownMenu(
                expanded = showMenu,
                onDismissRequest = { showMenu = false }
            ) {
                DropdownMenuItem(
                    text = { Text("Export") },
                    onClick = { showMenu = false }
                )
                DropdownMenuItem(
                    text = { Text("About") },
                    onClick = { showMenu = false }
                )
            }
        }
    )
}

@Composable
fun WgpuRenderSurface(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .background(Color(0xFF1E1E1E))
    ) {
        // Platform-specific rendering surface
        WgpuNativeView(
            modifier = Modifier.fillMaxSize()
        )
        
        // Overlay info
        Text(
            text = "WebGPU Render Surface",
            color = Color.White.copy(alpha = 0.5f),
            style = MaterialTheme.typography.labelSmall,
            modifier = Modifier
                .align(Alignment.TopStart)
                .padding(8.dp)
        )
    }
}

@Composable
fun InfoPanel() {
    var nativeInfo by remember { mutableStateOf("Loading...") }
    
    LaunchedEffect(Unit) {
        nativeInfo = try {
            NativeLib.getInfo()
        } catch (e: Exception) {
            "Error: ${e.message}"
        }
    }
    
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .height(120.dp),
        color = MaterialTheme.colorScheme.surfaceVariant
    ) {
        Column(
            modifier = Modifier
                .padding(16.dp)
                .fillMaxSize(),
            verticalArrangement = Arrangement.spacedBy(4.dp)
        ) {
            Text(
                text = "Native Info:",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Text(
                text = nativeInfo,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

// Platform-specific rendering view (expect/actual pattern)
@Composable
expect fun WgpuNativeView(modifier: Modifier = Modifier)