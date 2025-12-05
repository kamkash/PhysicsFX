# Rust wgpu Native Library Integration

## Overview

Successfully integrated a Rust library with wgpu support into the Kotlin Multiplatform project, with platform-specific bindings for:
- **JVM** (Java/Desktop) via JNI
- **Android** via JNI  
- **iOS** via C interop
- **Wasm** via wasm-bindgen (placeholder)

## What Was Done

### 1. Created Rust Library (`physics_core`)

Created a new Rust cargo project at [physics_core](file:///Users/kamran/PhysicsFX/physics_core) with:

#### [Cargo.toml](file:///Users/kamran/PhysicsFX/physics_core/Cargo.toml)
- Configured as `cdylib` and `staticlib` for multi-platform support
- Added dependencies: `wgpu`, `jni`, `wasm-bindgen`
- Created feature flags: `jni_support`, `wasm_support`

#### [lib.rs](file:///Users/kamran/PhysicsFX/physics_core/src/lib.rs)
Implemented three platform-specific interfaces:

**C Interface (for iOS)**:
```rust
pub extern "C" fn physics_core_get_info() -> *mut c_char
pub extern "C" fn physics_core_free_string(s: *mut c_char)
```

**JNI Interface (for Android/JVM)**:
```rust
pub extern "system" fn Java_app_kamkash_physicsfx_NativeLib_getInfo(
    env: JNIEnv, _class: JClass
) -> jni::sys::jstring
```

**Wasm Interface (for Web)**:
```rust
#[wasm_bindgen]
pub fn wasm_get_info() -> String
```

### 2. Kotlin Multiplatform Bindings

Created expect/actual pattern for cross-platform access:

#### Common ([NativeLib.kt](file:///Users/kamran/PhysicsFX/composeApp/src/commonMain/kotlin/app/kamkash/physicsfx/NativeLib.kt))
```kotlin
expect object NativeLib {
    fun getInfo(): String
}
```

#### Platform Implementations

**Android** - [androidMain/NativeLib.kt](file:///Users/kamran/PhysicsFX/composeApp/src/androidMain/kotlin/app/kamkash/physicsfx/NativeLib.kt)
- Uses JNI to call Rust
- Loads `libphysics_core.so` via `System.loadLibrary()`

**JVM** - [jvmMain/NativeLib.kt](file:///Users/kamran/PhysicsFX/composeApp/src/jvmMain/kotlin/app/kamkash/physicsfx/NativeLib.kt)
- Uses JNI with fallback loading strategies
- Tries multiple paths to find the native library
- Successfully loads from `../physics_core/target/release/libphysics_core.dylib`

**iOS** - [iosMain/NativeLib.kt](file:///Users/kamran/PhysicsFX/composeApp/src/iosMain/kotlin/app/kamkash/physicsfx/NativeLib.kt)
- Uses Kotlin/Native cinterop
- Calls C functions via generated bindings

**WasmJS** - [wasmJsMain/NativeLib.kt](file:///Users/kamran/PhysicsFX/composeApp/src/wasmJsMain/kotlin/app/kamkash/physicsfx/NativeLib.kt)
- Placeholder implementation (Wasm integration requires additional setup)

### 3. Build Configuration

Updated [build.gradle.kts](file:///Users/kamran/PhysicsFX/composeApp/build.gradle.kts) with platform-specific build tasks:

#### **buildRustDesktop** (JVM)
- Compiles Rust with `--features jni_support`
- Copies `.dylib`/`.dll`/`.so` to `src/jvmMain/resources`
- Automatically runs before `jvmProcessResources`

#### **buildRustAndroid**
- Uses `cargo-ndk` for cross-compiling to Android targets
- Builds for: armv7, aarch64, i686, x86_64
- Copies libraries to `src/androidMain/jniLibs/{abi}`

#### **buildRustIOS**
- Builds for iOS targets: `aarch64-apple-ios`, `aarch64-apple-ios-sim`
- Links with framework via linkerOpts

#### **buildRustWasm**
- Uses `wasm-pack` to build for web
- Copies to `src/wasmJsMain/resources`

### 4. iOS cinterop Configuration

Created cinterop setup:
- [physics_core.h](file:///Users/kamran/PhysicsFX/physics_core/include/physics_core.h) - C header file
- [physics_core.def](file:///Users/kamran/PhysicsFX/composeApp/src/nativeInterop/cinterop/physics_core.def) - cinterop definition
- Configured in build.gradle.kts with `extraOpts` for header path

## Verification

### JVM Test
Created [NativeLibTest.kt](file:///Users/kamran/PhysicsFX/composeApp/src/jvmTest/kotlin/app/kamkash/physicsfx/NativeLibTest.kt):

```kotlin
@Test
fun testGetInfo() {
    val info = NativeLib.getInfo()
    println("Info: $info")
    assertTrue(info.contains("wgpu"))
}
```

**Test Result**: ✅ **PASSED**

Output from test run:
```
Successfully loaded from /Users/kamran/PhysicsFX/composeApp/../physics_core/target/release/libphysics_core.dylib
Info: Hello from Rust wgpu core!
```

## Platform Status

| Platform | Integration | Build Task | Status |
|----------|-------------|------------|---------|
| JVM/Desktop | JNI | `buildRustDesktop` | ✅ **Working** |
| Android | JNI | `buildRustAndroid` | ⚠️ Ready (needs cargo-ndk) |
| iOS | C interop | `buildRustIOS` | ⚠️ Ready (needs Rust targets) |
| Wasm/JS | wasm-bindgen | `buildRustWasm` | ⚠️ Placeholder |

## Next Steps

To fully enable all platforms:

1. **Android**: Install cargo-ndk
   ```bash
   cargo install cargo-ndk
   ```

2. **iOS**: Add Rust targets
   ```bash
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim
   ```

3. **Wasm**: Install wasm-pack and implement JS module initialization
   ```bash
   cargo install wasm-pack
   ```

4. **Enable cinterop commonization** (optional):
   Add to `gradle.properties`:
   ```
   kotlin.mpp.enableCInteropCommonization=true
   ```

## Build Commands

```bash
# Build for JVM/Desktop and run tests
./gradlew buildRustDesktop jvmTest --no-configuration-cache

# Build for all platforms
./gradlew buildRustDesktop buildRustAndroid buildRustIOS buildRustWasm
```

## Files Created/Modified

### New Files
- `physics_core/` - Rust library
  - `Cargo.toml`
  - `src/lib.rs`
  - `include/physics_core.h`
- `composeApp/src/commonMain/kotlin/app/kamkash/physicsfx/NativeLib.kt`
- `composeApp/src/androidMain/kotlin/app/kamkash/physicsfx/NativeLib.kt`
- `composeApp/src/jvmMain/kotlin/app/kamkash/physicsfx/NativeLib.kt`
- `composeApp/src/iosMain/kotlin/app/kamkash/physicsfx/NativeLib.kt`
- `composeApp/src/wasmJsMain/kotlin/app/kamkash/physicsfx/NativeLib.kt`
- `composeApp/src/jvmTest/kotlin/app/kamkash/physicsfx/NativeLibTest.kt`
- `composeApp/src/nativeInterop/cinterop/physics_core.def`

### Modified Files
- [build.gradle.kts](file:///Users/kamran/PhysicsFX/composeApp/build.gradle.kts) - Added Rust build tasks and iOS configuration


- rustup target add armv7-linux-androideabi
- rustup target add aarch64-linux-android
- rustup target add i686-linux-android
- rustup target add x86_64-linux-android
- rustup target add x86_64-apple-darwin
- rustup target add aarch64-apple-ios
- rustup target add x86_64-apple-ios
- rustup target add aarch64-apple-ios-sim

