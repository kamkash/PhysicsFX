package app.kamkash.physicsfx

actual object NativeLib {
    init {
        try {
            System.loadLibrary("physics_core")
        } catch (e: UnsatisfiedLinkError) {
            println("Failed to load physics_core from java.library.path: ${e.message}")
            val osName = System.getProperty("os.name").lowercase()
            val libName = if (osName.contains("mac")) "libphysics_core.dylib" 
                          else if (osName.contains("win")) "physics_core.dll" 
                          else "libphysics_core.so"
            
            val paths = listOf(
                "physics_core/target/release/$libName",
                "../physics_core/target/release/$libName",
                "composeApp/src/jvmMain/resources/$libName",
                "src/jvmMain/resources/$libName"
            )
            
            var loaded = false
            for (p in paths) {
                val file = java.io.File(p).absoluteFile
                if (file.exists()) {
                    try {
                        System.load(file.path)
                        loaded = true
                        println("Successfully loaded from ${file.path}")
                        break
                    } catch (e2: Throwable) {
                        println("Failed to load from ${file.path}: ${e2.message}")
                    }
                }
            }
            if (!loaded) {
                throw UnsatisfiedLinkError("Could not load physics_core. CWD: ${java.io.File(".").absolutePath}")
            }
        }
    }
    external actual fun getInfo(): String
}
