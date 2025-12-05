package app.kamkash.physicsfx

import kotlin.test.Test
import kotlin.test.assertTrue

class NativeLibTest {
    @Test
    fun testGetInfo() {
        val info = NativeLib.getInfo()
        println("Info: $info")
        assertTrue(info.contains("wgpu"))
    }
}
