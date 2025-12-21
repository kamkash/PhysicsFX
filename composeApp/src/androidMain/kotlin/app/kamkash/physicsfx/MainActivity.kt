package app.kamkash.physicsfx

import android.os.Bundle
import android.view.SurfaceView
import android.view.ViewGroup
import android.widget.LinearLayout
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.platform.ComposeView
import com.google.androidgamesdk.GameActivity

class MainActivity : GameActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Find the SurfaceView managed by GameActivity
        // GameActivity usually has a FrameLayout as its content root, with a SurfaceView inside it.
        val rootView = window.decorView.findViewById<ViewGroup>(android.R.id.content)
        val surfaceView = findSurfaceView(rootView)

        if (surfaceView != null) {
            // Remove the SurfaceView from its original parent
            val originalParent = surfaceView.parent as? ViewGroup
            originalParent?.removeView(surfaceView)

            // Create a horizontal split layout
            val splitLayout =
                    LinearLayout(this).apply {
                        orientation = LinearLayout.HORIZONTAL
                        layoutParams =
                                ViewGroup.LayoutParams(
                                        ViewGroup.LayoutParams.MATCH_PARENT,
                                        ViewGroup.LayoutParams.MATCH_PARENT
                                )
                    }

            // Add the SurfaceView to the left (70%)
            surfaceView.layoutParams =
                    LinearLayout.LayoutParams(0, ViewGroup.LayoutParams.MATCH_PARENT, 7f)
            splitLayout.addView(surfaceView)

            // Add a splitter/border
            val splitter =
                    android.view.View(this).apply {
                        layoutParams =
                                LinearLayout.LayoutParams(1, ViewGroup.LayoutParams.MATCH_PARENT)
                        setBackgroundColor(android.graphics.Color.parseColor("#444444"))
                    }
            splitLayout.addView(splitter)

            // Add the ComposeView to the right (30%)
            val composeView =
                    ComposeView(this).apply {
                        id = android.view.View.generateViewId()
                        layoutParams =
                                LinearLayout.LayoutParams(
                                        0,
                                        ViewGroup.LayoutParams.MATCH_PARENT,
                                        3f
                                )
                    }
            splitLayout.addView(composeView)

            // Set the split layout as the new content view
            setContentView(splitLayout)

            // Initialize Compose content
            setupComposeContent(composeView)
        } else {
            android.util.Log.e("MainActivity", "Could not find native SurfaceView!")
        }
    }

    private fun findSurfaceView(view: ViewGroup): SurfaceView? {
        for (i in 0 until view.childCount) {
            val child = view.getChildAt(i)
            if (child is SurfaceView) return child
            if (child is ViewGroup) {
                val found = findSurfaceView(child)
                if (found != null) return found
            }
        }
        return null
    }

    private fun setupComposeContent(composeView: ComposeView) {
        composeView.setContent { SidePanel() }
    }

    companion object {
        init {
            System.loadLibrary("physics_core")
        }
    }
}
