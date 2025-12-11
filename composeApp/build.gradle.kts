import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.ExperimentalWasmDsl
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.plugin.mpp.NativeBinary
import java.io.ByteArrayOutputStream

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
    alias(libs.plugins.composeHotReload)
}

kotlin {
    androidTarget {
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_11)
        }
    }
    // Map Kotlin iOS targets to Rust target triples
    val iosTargetToRustTarget = mapOf(
        "iosArm64" to "aarch64-apple-ios",
        "iosSimulatorArm64" to "aarch64-apple-ios-sim"
    )
    
    listOf(
        iosArm64(),
        iosSimulatorArm64()
    ).forEach { iosTarget ->
        iosTarget.binaries.framework {
            baseName = "ComposeApp"
            isStatic = true
        }
        iosTarget.binaries.all {
            val rustTarget = iosTargetToRustTarget[iosTarget.name] 
                ?: throw GradleException("Unknown iOS target: ${iosTarget.name}")
            val physicsLibPath =
                rootProject.projectDir.resolve("physics_core/target/$rustTarget/release").absolutePath
            linkerOpts.add("-L$physicsLibPath")
            linkerOpts.add("-lphysics_core")
        }
        val main = iosTarget.compilations.getByName("main")
        main.cinterops.create("physics_core") {
            defFile(project.file("src/iosMain/cinterop/physics_core.def"))
            val rustIncludePath =
                rootProject.projectDir.resolve("physics_core/include").absolutePath
            includeDirs(rustIncludePath)
        }
    }

    jvm()
    
    js {
        browser()
        binaries.executable()
    }
    
    @OptIn(ExperimentalWasmDsl::class)
    wasmJs {
        browser()
        binaries.executable()
    }
    
    sourceSets {
        androidMain.dependencies {
            implementation(libs.androidx.appcompat)
            implementation(libs.androidx.lifecycle.viewmodelCompose)
            implementation(libs.androidx.lifecycle.runtimeCompose)
        }
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material3)
            implementation(compose.ui)
            implementation(compose.components.resources)
            implementation(compose.components.uiToolingPreview)
        }
        commonTest.dependencies {
            implementation(libs.kotlin.test)
        }
        jvmMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.kotlinx.coroutinesSwing)
            implementation("net.java.dev.jna:jna:5.13.0")
            implementation("net.java.dev.jna:jna-platform:5.13.0")
        }
    }
}

android {
    namespace = "app.kamkash.physicsfx"
    compileSdk = libs.versions.android.compileSdk.get().toInt()
     buildFeatures {
        prefab = true
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/androidMain/jniLibs")
        }
    }

    defaultConfig {
        applicationId = "app.kamkash.physicsfx"
        minSdk = libs.versions.android.minSdk.get().toInt()
        targetSdk = libs.versions.android.targetSdk.get().toInt()
        versionCode = 1
        versionName = "1.0"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

dependencies {
    debugImplementation(compose.uiTooling)
    implementation(libs.androidx.games.activity)
    implementation(libs.androidx.appcompat)
}

compose.desktop {
    application {
        mainClass = "app.kamkash.physicsfx.MainKt"

        jvmArgs += listOf(
            "-Djava.library.path=${project.projectDir}/src/jvmMain/resources",
            "--add-exports", "java.desktop/sun.awt=ALL-UNNAMED",
            "--add-exports", "java.desktop/sun.lwawt=ALL-UNNAMED",
            "--add-exports", "java.desktop/sun.lwawt.macosx=ALL-UNNAMED", 
            "--add-opens", "java.desktop/sun.awt=ALL-UNNAMED",
            "--add-opens", "java.desktop/java.awt=ALL-UNNAMED",
            "-XstartOnFirstThread",
            "-Dsun.java2d.metal=true"
        )

        nativeDistributions {
            targetFormats(TargetFormat.Dmg, TargetFormat.Msi, TargetFormat.Deb)
            packageName = "app.kamkash.physicsfx"
            packageVersion = "1.0.0"
        }
    }
}

tasks.register("buildRustDesktop") {
    doLast {
        val rustDir = rootProject.projectDir.resolve("physics_core")
        val targetDir = rustDir.resolve("target/release")
        val resourcesDir = projectDir.resolve("src/jvmMain/resources")
        
        exec {
            workingDir = rustDir
            commandLine("cargo", "build", "--release", "--features", "jni_support")
        }
        val osName = System.getProperty("os.name").lowercase()
        val libName = if (osName.contains("mac")) "libphysics_core.dylib" 
                      else if (osName.contains("win")) "physics_core.dll" 
                      else "libphysics_core.so"
        copy {
            from(targetDir.resolve(libName))
            into(resourcesDir)
        }
    }
}

tasks.register("buildRustWasm") {
    doLast {
        val rustDir = rootProject.projectDir.resolve("physics_core")
        val pkgDir = rustDir.resolve("pkg")
        val resourcesDir = projectDir.resolve("src/wasmJsMain/resources")

        exec {
            workingDir = rustDir
            commandLine("wasm-pack", "build", "--target", "web", "--features", "wasm_support")
        }
        copy {
            from(pkgDir)
            into(resourcesDir)
        }
    }
}

tasks.register("buildRustIOS") {
    doLast {
        val rustDir = rootProject.projectDir.resolve("physics_core")
        val cargoHome = System.getProperty("user.home") + "/.cargo/bin"
        val cargo = "$cargoHome/cargo"
        val rustup = "$cargoHome/rustup"
        
        // Fetch SDKROOT for macOS
        val sdkRoot = try {
            val stdout = ByteArrayOutputStream()
            exec {
                commandLine("xcrun", "--sdk", "macosx", "--show-sdk-path")
                standardOutput = stdout
            }
            stdout.toString().trim()
        } catch (e: Exception) {
            println("Failed to get SDK path via xcrun: $e")
            ""
        }
        
        val targets = listOf("aarch64-apple-ios", "aarch64-apple-ios-sim")
        for (target in targets) {
            exec {
                workingDir = rustDir
                environment("PATH", System.getenv("PATH") + ":$cargoHome") // Ensure tools are found
                commandLine(rustup, "target", "add", target)
            }
            exec {
                workingDir = rustDir
                environment("PATH", System.getenv("PATH") + ":$cargoHome")
                if (sdkRoot.isNotEmpty()) {
                    environment("SDKROOT", sdkRoot)
                }
                commandLine(cargo, "build", "--release", "--target", target)
            }
        }
    }
}

tasks.register("buildRustAndroid") {
    doLast {
        val rustDir = rootProject.projectDir.resolve("physics_core")
        
        // Check if cargo-ndk is available, if not provide instructions
        try {
            exec {
                commandLine("cargo", "ndk", "--version")
            }
        } catch (e: Exception) {
            println("cargo-ndk not found. Install with: cargo install cargo-ndk")
            println("Also ensure Android NDK is installed via Android Studio")
            return@doLast
        }
        
        val targets = listOf("armv7-linux-androideabi", "aarch64-linux-android", "i686-linux-android", "x86_64-linux-android")
        for (target in targets) {
            exec {
                workingDir = rustDir
                commandLine("cargo", "ndk", "--target", target, "--platform", "24", "build", "--release")
            }
        }
        
        // Copy libraries to jniLibs
        val ndkTargets = mapOf(
            "armv7-linux-androideabi" to "armeabi-v7a",
            "aarch64-linux-android" to "arm64-v8a",
            "i686-linux-android" to "x86",
            "x86_64-linux-android" to "x86_64"
        )
        
        for ((rustTarget, androidAbi) in ndkTargets) {
            val srcLib = rustDir.resolve("target/$rustTarget/release/libphysics_core.so")
            val jniLibsDir = projectDir.resolve("src/androidMain/jniLibs/$androidAbi")
            copy {
                from(srcLib)
                into(jniLibsDir)
            }
        }
    }
}

tasks.named("jvmProcessResources") {
    dependsOn("buildRustDesktop")
}

tasks.named("wasmJsProcessResources") {
    dependsOn("buildRustWasm")
}

// Hook iOS build
tasks.matching { it.name.contains("link") && it.name.contains("Framework") }.configureEach {
    dependsOn("buildRustIOS")
}

// Hook Android build
tasks.matching { it.name.contains("mergeDebugJniLibFolders") || it.name.contains("mergeReleaseJniLibFolders") }.configureEach {
    dependsOn("buildRustAndroid")
}
