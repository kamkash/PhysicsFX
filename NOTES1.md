## Key commands

### Gradle / Kotlin Multiplatform

Run all commands from the repo root unless noted.

- Desktop (JVM) wgpu demo (uses Rust winit via `JvmWgpuGameLoop`):
  - `./gradlew :composeApp:run`
- Android debug APK:
  - Build APK: `./gradlew :composeApp:assembleDebug`
  - Install to connected device (from README):
    - Ensure Android platform tools on PATH, then:
      - `adb devices -l`
      - `adb install ./composeApp/build/outputs/apk/debug/composeApp-debug.apk`
      - Or to target a specific device: `adb install -s <deviceId> ./composeApp/build/outputs/apk/debug/composeApp-debug.apk`
- Web (preferred Wasm target):
  - `./gradlew :composeApp:wasmJsBrowserDevelopmentRun`
- Web (JS fallback):
  - `./gradlew :composeApp:jsBrowserDevelopmentRun`
- Android/iOS framework builds are wired into Gradle; the main iOS frameworks are produced by:
  - `./gradlew :composeApp:linkDebugFrameworkIosSimulatorArm64`
  - `./gradlew :composeApp:linkDebugFrameworkIosArm64`

#### Rust build helpers (invoked automatically by Gradle)

`composeApp/build.gradle.kts` defines tasks that build the Rust core for each platform and are hooked into the Kotlin build:

- Desktop (JVM JNI library):
  - Task: `./gradlew :composeApp:buildRustDesktop`
  - Internally: `cargo build --release --features jni_support` in `physics_core`, then copies the platform-specific shared library into `composeApp/src/jvmMain/resources`.
- Web (Wasm):
  - Task: `./gradlew :composeApp:buildRustWasm`
  - Internally: `wasm-pack build --target web --features wasm_support` in `physics_core`, then copies the generated `pkg` into `composeApp/src/wasmJsMain/resources`.
- iOS (static libraries for Compose iOS frameworks):
  - Task: `./gradlew :composeApp:buildRustIOS`
  - Uses `rustup target add aarch64-apple-ios aarch64-apple-ios-sim` and `cargo build --release --target <ios-target>` inside `physics_core`.
  - The resulting archives are linked into iOS frameworks via `linkerOpts` in the iOS targets.
- Android (JNI `.so` for ABIs):
  - Task: `./gradlew :composeApp:buildRustAndroid`
  - Uses `cargo ndk --target <abi> --platform 24 build --release --features jni_support` in `physics_core` and copies `.so` files into `composeApp/src/androidMain/jniLibs/<abi>`.

You normally do **not** need to invoke these tasks directly; they are wired as dependencies of the relevant Gradle tasks:

- `jvmProcessResources` depends on `buildRustDesktop`.
- `wasmJsProcessResources` depends on `buildRustWasm`.
- All iOS framework link tasks depend on `buildRustIOS`.
- Android JNI merge tasks depend on `buildRustAndroid`.

Run them explicitly when debugging Rust build issues (
for example, `./gradlew :composeApp:buildRustDesktop` to see pure Rust errors without going through the full JVM build).

### Rust-only workflows (`physics_core` crate)

From `physics_core/`:

- Build native+JNI library for desktop/Android/JVM:
  - `cargo build --release --features jni_support`
- Build WebAssembly package (if you want to bypass Gradle):
  - `wasm-pack build --target web --features wasm_support`
- Run Rust tests:
  - `cargo test`

### iOS app workflows (SwiftUI wrapper)

The Compose UI is packaged as a framework and embedded in a native SwiftUI app under `iosApp/`.

Typical flows (see `README.md` for full CLI variants and device-specific commands):

- Open in Xcode (recommended):
  - `open iosApp/iosApp.xcodeproj`
  - Select a simulator or device and run (Cmd+R).
- Build simulator frameworks via Gradle (before using Xcode if needed):
  - `./gradlew :composeApp:linkDebugFrameworkIosSimulatorArm64`

The README contains detailed `xcodebuild`, `xcrun`, and `ios-deploy` command sequences for simulators and a specific physical iPad; consult it when you need reproducible CLI builds.

### Tests

There is a small JVM-side test suite validating the Rust bridge:

- Run all JVM tests for `composeApp`:
  - `./gradlew :composeApp:jvmTest`
- Run a single JVM test (example):
  - `./gradlew :composeApp:jvmTest --tests 'app.kamkash.physicsfx.NativeLibTest.testGetInfo'`

Rust tests live in `physics_core` and are run with `cargo test` as shown above.

### Lint / checks

The Android plugin provides standard lint and check tasks:

- Run Android lint for the `composeApp` module (debug variant):
  - `./gradlew :composeApp:lintDebug`
- Run general Gradle checks for `composeApp`:
  - `./gradlew :composeApp:check`

## High-level architecture

### Overview

PhysicsFX is a Kotlin Multiplatform project that uses a Rust `wgpu` core for rendering/physics and exposes that core to Android, desktop JVM, iOS, and web targets through:

- A shared Kotlin API surface in `composeApp/src/commonMain/kotlin/app/kamkash/physicsfx`.
- Platform-specific Kotlin implementations per target (Android, JVM, iOS, JS/Wasm).
- A Rust crate `physics_core` that owns all `wgpu` setup and rendering and exports a C/FFI, JNI, and wasm-bindgen API.
- Native SwiftUI wrapper app in `iosApp/` that hosts the Compose UI on iOS.

The key documents describing the architecture and game loop are:

- `GAME_LOOP.md` – cross-platform game loop design and per-platform implementations.
- `NOTES.md` – high-level explanation of the source set layout and Rust integration.
- `NOTES1.md` – description of the WebGPU-oriented UI structure and `WgpuNativeView` behavior.

### Kotlin shared layer (`composeApp`)

The shared Compose UI and platform abstractions live under `composeApp/src/commonMain/kotlin/app/kamkash/physicsfx/`:

- `App.kt` defines the main UI:
  - A `Scaffold` with `MainToolbar` (app title and placeholder controls).
  - A `Row` with two main regions:
    - `WgpuRenderSurface` – a dark background area that hosts the platform-specific `WgpuNativeView` where the WebGPU/wgpu content will render.
    - `InfoPanel` – calls `NativeLib.getInfo()` and shows the string returned from Rust ("Hello from Rust wgpu core!"), or an error message.
  - `WgpuNativeView` is declared as an `expect` composable, to be implemented per target.

- `NativeLib` (expect/actual in commonMain and target-specific source sets):
  - In `commonMain`, `expect object NativeLib { fun getInfo(): String }` defines the shared API.
  - In `jvmMain`, `actual object NativeLib` loads the `physics_core` native library:
    - First via `System.loadLibrary("physics_core")` from `java.library.path`.
    - If that fails, it searches several fallback paths (including `physics_core/target/release` and `src/jvmMain/resources`) and calls `System.load(...)` on the first matching file.
    - Exposes `external actual fun getInfo(): String`, which is implemented in Rust via JNI.
  - Other targets (Android, iOS, JS/Wasm) have their own `actual` implementations that bind to the appropriate Rust export for that platform.

- `WgpuGameLoop` (`WgpuGameLoop.kt`) defines a single, platform-agnostic interface that mirrors the Rust C/FFI API:
  - `start(surfaceHandle: Any?, width: Int, height: Int)` – initialize `wgpu` and create the rendering surface.
  - `update(deltaTime: Float)` – update game/physics state.
  - `render()` – draw a frame.
  - `resize(width, height)` – handle window/surface size changes.
  - `end()` – clean up and shut down.
  - `isRunning(): Boolean` – track loop lifecycle state.

Concrete `WgpuGameLoop` implementations live in platform-specific source sets and call into the Rust `wgpu_*` API via JNI, C, or wasm-bindgen as appropriate (see `GAME_LOOP.md` for a detailed list and behavior of `JvmWgpuGameLoop`, `AndroidWgpuGameLoop`, `IosWgpuGameLoop`, and `WasmWgpuGameLoop`).

### Platform-specific rendering surfaces (`WgpuNativeView.*`)

Each target implements `WgpuNativeView` as the bridge between Compose UI and a native rendering surface that Rust/wgpu can render into.

#### Android (`WgpuNativeView.android.kt`)

- Uses `AndroidView` to host an Android `SurfaceView`.
- Creates an `AndroidWgpuGameLoop` instance and ties it to the `SurfaceView` lifecycle via `SurfaceHolder.Callback`:
  - `surfaceCreated` → calls `gameLoop.start(holder.surface, width, height)` once the view has a non-zero size.
  - `surfaceChanged` → either resizes an existing loop or starts it if it was not yet running.
  - `surfaceDestroyed` → calls `gameLoop.end()`.
- `DisposableEffect` ensures `gameLoop.end()` is called when the composable leaves composition, even if the Android view is not explicitly destroyed.
- On the Rust side, the JNI bindings convert the Android `Surface` into an `ANativeWindow` pointer and pass it to `wgpu_init`.

#### iOS (`WgpuNativeView.ios.kt`)

- Defines a custom `MetalView : UIView` whose `layerClass` is `CAMetalLayer`, giving full control over Metal rendering.
- `MetalView` holds a `WgpuGameLoop` reference and overrides `layoutSubviews` to compute pixel dimensions (frame size × `contentScaleFactor`) and call `loop?.resize(width, height)`.
- `WgpuNativeView` uses `UIKitView` to embed `MetalView` in Compose:
  - In `factory`, it creates a `MetalView`, configures `contentScaleFactor`, touch, and background color, then:
    - Computes initial pixel size.
    - Obtains a raw pointer to the `UIView` (`view.objcPtr()` and `interpretCPointer`) and passes it to `gameLoop.start(viewPtr, width, height)`.
  - In `update`, it checks if the game loop is not running and the view has a non-zero pixel size; if so, it starts the loop with the updated size.
  - `DisposableEffect` and `onRelease` both call `gameLoop.end()`.
- On the Rust side, `wgpu_init` uses `raw_window_handle`'s `UiKitWindowHandle` / `UiKitDisplayHandle` with the passed pointer to create a `wgpu::Surface`.

#### JVM / Desktop (`WgpuNativeView.jvm.kt` and `Main.kt`)

- `WgpuNativeView.jvm.kt` currently provides a placeholder implementation that simply renders a centered "Rendering via Winit Window" text; the desktop rendering is driven by a separate entry point that uses a native window instead of an embedded Compose view.
- `Main.kt` in `jvmMain` is the desktop entry and currently delegates to Rust-driven rendering via a standalone window:
  - Calls `JvmWgpuGameLoop().runWinit()`.
  - That implementation (see `GAME_LOOP.md`) ultimately invokes the Rust `start_winit_app()` function exposed from `physics_core`, which:
    - Creates a `winit` window.
    - Uses its `HasWindowHandle`/`HasDisplayHandle` to derive raw handles.
    - Calls `init_wgpu_internal` to set up `wgpu` and enters a `winit` event loop that triggers `render_internal()` on resize/redraw events.

This separation makes it easy to experiment with and debug the Rust rendering stack in isolation from Compose Desktop.

#### Web / Wasm (`WgpuNativeView.wasmJs.kt` and `composeApp/src/webMain/resources/index.html`)

- `index.html` contains a minimal page that loads `composeApp.js` and shows a simple loading spinner.
- `WgpuNativeView.wasmJs.kt` manages an out-of-tree `<canvas>` element for wgpu/WebGPU:
  - On first composition, creates a canvas with a fixed ID (`wgpu-canvas-overlay`), styles it as a fixed overlay positioned under the toolbar, and appends it directly to `document.documentElement` to avoid issues with Compose’s shadow DOM.
  - On dispose, removes the canvas from the DOM.
  - Wraps the Composable content in a `Box` whose `modifier.onSizeChanged` block:
    - Observes the Compose layout size, logs it, and retrieves the canvas element.
    - Uses a clamped width/height (currently a fixed 400×400, with commented-out logic for matching the composable size) and calls:
      - `gameLoop.start(canvasId, width, height)` the first time.
      - `gameLoop.resize(width, height)` on subsequent size changes while the loop is running.
- `WasmWgpuGameLoop` (see `GAME_LOOP.md`) calls into the Rust wasm-bindgen exports `wasm_init`, `wasm_update`, `wasm_render`, `wasm_resize`, and `wasm_shutdown` which are re-exported from the generated `physics_core.js` in `wasmJsMain/resources`.
- On the Rust side (`wasm_support` feature):
  - `wasm_init(canvas_id, width, height)` fetches the canvas element by ID, creates a WebGPU surface when possible, and falls back to a WebGL-backed path if WebGPU initialization fails.
  - The same `WgpuState` and rendering pipeline are used as on native platforms.

### Rust core (`physics_core`)

The `physics_core` crate encapsulates all `wgpu` logic and the cross-platform rendering APIs.

#### Crate configuration (`Cargo.toml`)

- Library types: `cdylib` and `staticlib` to support dynamic loading (JNI/desktop/Android) and static linking (iOS).
- Core dependencies:
  - `wgpu` (with `webgl` feature), `pollster`, `raw-window-handle`.
  - Logging: `log`, `env_logger`.
  - Utilities: `bytemuck` (for vertex data), `once_cell`.
- Target-specific dependencies:
  - Android: `ndk`, `ndk-sys`.
  - Wasm: `web-sys`, `wasm-bindgen-futures`, `console_log`, `console_error_panic_hook`.
  - Non-Android/non-Wasm desktop: `winit`.
- Features:
  - `jni_support` – enables JNI bindings for Android/JVM.
  - `wasm_support` – enables wasm-bindgen exports and WebGPU/WebGL initialization path.

#### Core rendering structures (`src/lib.rs`)

- `Vertex` struct and static `VERTICES`/`INDICES` define a simple textured triangle.
- `WgpuState` holds the `wgpu::Instance`, `Device`, `Queue`, `Surface`, `SurfaceConfiguration`, `RenderPipeline`, vertex/index buffers, and a texture bind group.
- A global `Lazy<Mutex<WgpuStateWrapper>>` plus `INITIALIZED: AtomicBool` manage cross-platform lifecycle.
- `init_wgpu_internal(window_handle, display_handle, width, height)` performs the common initialization path:
  - Creates a `wgpu::Instance` and unsafe `wgpu::Surface` from a type that implements `HasWindowHandle` and `HasDisplayHandle` (wrapping raw handles passed in from each platform).
  - Picks a suitable adapter and device, using conservative limits for WebGL-like environments and adapter limits when WebGPU is available.
  - Configures the surface, creates a procedural texture, shader module (from `shader.wgsl`), bind groups, and render pipeline.
  - Creates vertex/index buffers and stores everything in `WGPU_STATE`.

- `render_internal()` acquires the current surface texture, begins a render pass, clears it to green, draws the triangle, and presents the frame.
- `resize_internal(width, height)` clamps dimensions by `max_texture_dimension_2d`, reconfigures the surface, and updates the stored configuration.
- `shutdown_internal()` drops the stored state and resets `INITIALIZED`.

The same internal functions are used by all platform entry points.

#### C/FFI, JNI, and wasm-bindgen interfaces

- C / iOS / generic native:
  - `physics_core_get_info()` and `physics_core_free_string()` – simple info string and deallocator.
  - `wgpu_init(surface_handle: *mut c_void, width: i32, height: i32) -> bool` – creates platform-specific `RawWindowHandle`/`RawDisplayHandle` based on the current target (UiKit/AppKit/Win32/Xlib/Android) and then calls `init_wgpu_internal`.
  - `wgpu_update(delta_time: f32)`, `wgpu_render()`, `wgpu_resize(width, height)`, and `wgpu_shutdown()` – thin wrappers around the internal functions.

- JNI (Android & JVM desktop), gated by `jni_support`:
  - Exposes `Java_app_kamkash_physicsfx_NativeLib_getInfo` to back the Kotlin `NativeLib.getInfo()` in `jvmMain`.
  - Provides `Java_app_kamkash_physicsfx_JvmWgpuGameLoop_native*` and `Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_native*` functions for game loop lifecycle (init/update/render/resize/shutdown).
  - On Android, converts a Java `Surface` into an `ANativeWindow` via `ndk_sys::ANativeWindow_fromSurface` before calling `wgpu_init`.

- Wasm (`wasm_support`):
  - `wasm_get_info()` mirrors `get_internal_info` for the JS world.
  - `wasm_init(canvas_id, width, height)` performs asynchronous adapter/device acquisition and sets up the `WgpuState` using a `GpuCanvasContext` from the DOM canvas.
  - `wasm_update`, `wasm_render`, `wasm_resize`, `wasm_shutdown` delegate to the same internal logic as native platforms, with additional logging and texture size clamping.
  - These are wrapped by `physics_core.js` (generated by wasm-bindgen and placed in `composeApp/src/wasmJsMain/resources`) and consumed from Kotlin via `WasmWgpuGameLoop`.

- Winit desktop app (`start_winit_app`):
  - For macOS/Windows/Linux (non-Android/non-Wasm), there is a standalone `start_winit_app()` function that creates a `winit` window and uses `init_wgpu_internal` and `render_internal` directly in an event loop.
  - JNI exposes `Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeStartWinitApp`, which `JvmWgpuGameLoop` calls from the JVM entry point in `Main.kt`.

### iOS SwiftUI wrapper (`iosApp`)

- `iosApp/iosApp.swift` defines the `@main` SwiftUI app:
  - Wraps `ContentView()` inside a `WindowGroup`.
- `ContentView.swift` bridges SwiftUI and Compose:
  - `ComposeView : UIViewControllerRepresentable` returns `MainViewControllerKt.MainViewController()` from the shared Kotlin code.
  - `ContentView` simply hosts `ComposeView().ignoresSafeArea()`.
- The resulting app binary links against the `ComposeApp.framework` built by the KMP/Gradle iOS targets, which in turn statically link `physics_core` via the `cinterop` configuration and linker options in `composeApp/build.gradle.kts`.

## When editing or extending this project

- For **game loop or rendering changes**, keep the shared `WgpuGameLoop` contract and update all platform-specific implementations (`JvmWgpuGameLoop`, `AndroidWgpuGameLoop`, `IosWgpuGameLoop`, `WasmWgpuGameLoop`) together. See `GAME_LOOP.md` for current behavior and lifecycle diagrams.
- For **UI changes that affect the render surface**, modify `App.kt`, `WgpuRenderSurface`, and/or the appropriate `WgpuNativeView.*` implementation. `NOTES1.md` documents the intended layout and how `WgpuNativeView` is supposed to evolve (e.g., replacing placeholders with real surfaces).
- For **Rust API extensions**, add functions to `physics_core/src/lib.rs`, update `physics_core/include/physics_core.h`, extend the JNI/C/wasm-bindgen surfaces as needed, and then mirror the new capabilities in `NativeLib`/`WgpuGameLoop` and their platform-specific implementations.
