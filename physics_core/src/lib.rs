use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "jni_support")]
use jni::JNIEnv;
#[cfg(feature = "jni_support")]
use jni::objects::JClass;

#[cfg(feature = "wasm_support")]
use wasm_bindgen::prelude::*;

// Global state for game loop
static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn get_internal_info() -> String {
    "Hello from Rust wgpu core!".to_string()
}

// --- C / iOS Interface ---

#[no_mangle]
pub extern "C" fn physics_core_get_info() -> *mut c_char {
    let s = get_internal_info();
    let c_str = CString::new(s).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn physics_core_free_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

// Game loop lifecycle - C interface
#[no_mangle]
pub extern "C" fn wgpu_init(width: i32, height: i32) -> bool {
    log::info!("wgpu_init called: {}x{}", width, height);
    // TODO: Initialize wgpu device, surface, swapchain
    INITIALIZED.store(true, Ordering::Relaxed);
    true
}

#[no_mangle]
pub extern "C" fn wgpu_update(delta_time: f32) {
    if !INITIALIZED.load(Ordering::Relaxed) { return; }
    // TODO: Update game logic
    log::trace!("wgpu_update: dt={}", delta_time);
}

#[no_mangle]
pub extern "C" fn wgpu_render() {
    if !INITIALIZED.load(Ordering::Relaxed) { return; }
    // TODO: Render frame with wgpu
    log::trace!("wgpu_render called");
}

#[no_mangle]
pub extern "C" fn wgpu_resize(width: i32, height: i32) {
    if !INITIALIZED.load(Ordering::Relaxed) { return; }
    log::info!("wgpu_resize: {}x{}", width, height);
    // TODO: Recreate swapchain with new size
}

#[no_mangle]
pub extern "C" fn wgpu_shutdown() {
    log::info!("wgpu_shutdown called");
    // TODO: Cleanup wgpu resources
    INITIALIZED.store(false, Ordering::Relaxed);
}

// --- JNI Interface (Android & JVM) ---

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_NativeLib_getInfo(
    env: JNIEnv,
    _class: JClass,
) -> jni::sys::jstring {
    let info = get_internal_info();
    let output = env.new_string(info).expect("Couldn't create java string!");
    output.into_raw()
}

// Game loop lifecycle - JNI interface for JvmWgpuGameLoop
#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeInit(
    _env: JNIEnv,
    _class: JClass,
    width: jni::sys::jint,
    height: jni::sys::jint,
) -> jni::sys::jboolean {
    wgpu_init(width as i32, height as i32) as jni::sys::jboolean
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeUpdate(
    _env: JNIEnv,
    _class: JClass,
    delta_time: jni::sys::jfloat,
) {
    wgpu_update(delta_time as f32);
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeRender(
    _env: JNIEnv,
    _class: JClass,
) {
    wgpu_render();
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeResize(
    _env: JNIEnv,
    _class: JClass,
    width: jni::sys::jint,
    height: jni::sys::jint,
) {
    wgpu_resize(width as i32, height as i32);
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeShutdown(
    _env: JNIEnv,
    _class: JClass,
) {
    wgpu_shutdown();
}

// Game loop lifecycle - JNI interface for AndroidWgpuGameLoop
#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_nativeInit(
    _env: JNIEnv,
    _class: JClass,
    width: jni::sys::jint,
    height: jni::sys::jint,
) -> jni::sys::jboolean {
    wgpu_init(width as i32, height as i32) as jni::sys::jboolean
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_nativeUpdate(
    _env: JNIEnv,
    _class: JClass,
    delta_time: jni::sys::jfloat,
) {
    wgpu_update(delta_time as f32);
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_nativeRender(
    _env: JNIEnv,
    _class: JClass,
) {
    wgpu_render();
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_nativeResize(
    _env: JNIEnv,
    _class: JClass,
    width: jni::sys::jint,
    height: jni::sys::jint,
) {
    wgpu_resize(width as i32, height as i32);
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_AndroidWgpuGameLoop_nativeShutdown(
    _env: JNIEnv,
    _class: JClass,
) {
    wgpu_shutdown();
}

// --- Wasm Interface ---

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_get_info() -> String {
    get_internal_info()
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_init(width: u32, height: u32) -> bool {
    wgpu_init(width as i32, height as i32)
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_update(delta_time: f32) {
    wgpu_update(delta_time);
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_render() {
    wgpu_render();
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_resize(width: u32, height: u32) {
    wgpu_resize(width as i32, height as i32);
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_shutdown() {
    wgpu_shutdown();
}
