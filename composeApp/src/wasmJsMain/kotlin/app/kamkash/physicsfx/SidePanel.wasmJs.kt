package app.kamkash.physicsfx

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
actual fun SidePanel() {
    var nativeInfo by remember { mutableStateOf("Loading...") }

    LaunchedEffect(Unit) {
        nativeInfo =
                try {
                    NativeLib.getInfo()
                } catch (e: Exception) {
                    "Error: ${e.message}"
                }
    }

    Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.surfaceVariant) {
        Column(
                modifier = Modifier.padding(16.dp).fillMaxSize(),
                verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Text(text = "Physics Controls", style = MaterialTheme.typography.headlineSmall)

            HorizontalDivider()

            // Gravity Slider
            var gravity by remember { mutableStateOf(9.8f) }
            Column {
                Text(
                        text = "Gravity: ${gravity.format(1)} m/s²",
                        style = MaterialTheme.typography.bodyMedium
                )
                Slider(
                        value = gravity,
                        onValueChange = {
                            gravity = it
                            NativeLib.setGravity(it)
                        },
                        valueRange = 0f..20f
                )
            }

            // Simulation Speed
            var speed by remember { mutableStateOf(1.0f) }
            Column {
                Text(
                        text = "Time Scale: ${speed.format(1)}x",
                        style = MaterialTheme.typography.bodyMedium
                )
                Slider(
                        value = speed,
                        onValueChange = {
                            speed = it
                            NativeLib.setTimeScale(it)
                        },
                        valueRange = 0.1f..5f
                )
            }

            // Pause Toggle
            Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(text = "Pause Simulation", style = MaterialTheme.typography.bodyMedium)
                var paused by remember { mutableStateOf(false) }
                Switch(
                        checked = paused,
                        onCheckedChange = {
                            paused = it
                            NativeLib.setPaused(it)
                        }
                )
            }

            Button(onClick = { NativeLib.resetSimulation() }, modifier = Modifier.fillMaxWidth()) {
                Text("Reset Simulation")
            }

            Spacer(modifier = Modifier.weight(1f))

            Text(
                    text = "Native Info:",
                    style = MaterialTheme.typography.labelMedium,
            )
            Text(
                    text = nativeInfo,
                    style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

// Simple formatter for WASM since String.format might not be available
private fun Float.format(digits: Int): String {
    val multiplier = 10.0f
    return ((this * multiplier).toInt().toFloat() / multiplier).toString()
}
