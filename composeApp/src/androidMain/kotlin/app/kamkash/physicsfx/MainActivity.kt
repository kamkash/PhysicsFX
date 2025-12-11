package app.kamkash.physicsfx

import com.google.androidgamesdk.GameActivity

class MainActivity : GameActivity() {
    companion object {
        init {
            System.loadLibrary("physics_core")
        }
    }
}