use once_cell::sync::Lazy;
use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use std::ffi::c_void; // Needed for casting
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use wgpu::util::DeviceExt;

extern "C" {
    fn ANativeWindow_acquire(window: *mut c_void);
    fn ANativeWindow_getHeight(window: *mut c_void) -> i32;
    fn ANativeWindow_getWidth(window: *mut c_void) -> i32;
}


#[cfg(target_os = "android")]
use android_activity::{
    input::{InputEvent, MotionAction},
    AndroidApp, InputStatus, MainEvent, PollEvent,
};
#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(feature = "jni_support")]
use jni::objects::JClass;
#[cfg(feature = "jni_support")]
use jni::sys::{jboolean, jfloat, jint, jlong};
#[cfg(feature = "jni_support")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use log::LevelFilter;
#[cfg(target_os = "android")]
use ndk::native_window::NativeWindow;
#[cfg(target_os = "android")]
use raw_window_handle::{AndroidDisplayHandle, AndroidNdkWindowHandle};
#[cfg(feature = "wasm_support")]
use wasm_bindgen::prelude::*;

// Global state for game loop
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position: [f32; 2],
    velocity: [f32; 2],
    scale: f32,
    rotation: f32,
    uv: [f32; 2],
}

impl Instance {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // velocity - skipped in VS, but present in buffer
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // scale
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 2]>() * 2) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                // rotation
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 2]>() * 2 + std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                // uv
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 2]>() * 2 + std::mem::size_of::<f32>() * 2) as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        tex_coords: [0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2];

/// Holds the wgpu state for rendering
struct WgpuState {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline, // NEW
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,           // NEW
    diffuse_bind_group: wgpu::BindGroup,
    compute_bind_group: wgpu::BindGroup,     // NEW
    num_instances: u32,                  // NEW
    window_ptr: *mut c_void, // Debug: track window pointer

    #[cfg(target_arch = "wasm32")]
    last_render_time: f64,
    #[cfg(not(target_arch = "wasm32"))]
    last_render_time: std::time::Instant,

    frame_count: u32,
    #[cfg(target_arch = "wasm32")]
    last_fps_log_time: f64,
    #[cfg(not(target_arch = "wasm32"))]
    last_fps_log_time: std::time::Instant,
}

// Wrapper to force Send/Sync for WASM where we know it's single-threaded
struct WgpuStateWrapper(Option<WgpuState>);

unsafe impl Send for WgpuStateWrapper {}
unsafe impl Sync for WgpuStateWrapper {}
// unsafe impl Send for WgpuState {} // WgpuState contains *mut c_void which is !Send

static WGPU_STATE: Lazy<Mutex<WgpuStateWrapper>> = Lazy::new(|| Mutex::new(WgpuStateWrapper(None)));

fn get_internal_info() -> String {
    "Hello from Rust wgpu core!".to_string()
}

// --- Surface Handle Wrapper for raw pointers ---

/// Wrapper to implement HasWindowHandle/HasDisplayHandle for raw pointers
struct RawSurfaceHandle {
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl HasWindowHandle for RawSurfaceHandle {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        // SAFETY: The caller guarantees the handle is valid for the lifetime of the surface
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(self.window_handle) })
    }
}

impl HasDisplayHandle for RawSurfaceHandle {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        // SAFETY: The caller guarantees the handle is valid for the lifetime of the surface
        Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(self.display_handle) })
    }
}

fn create_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::TextureView, wgpu::Sampler) {
    let width = 256u32;
    let height = 256u32;

    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Diffuse Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = ((x ^ y) as u8).wrapping_mul(4); // Checkerboard-ish pattern
            data.extend_from_slice(&[r, g, b, 255]);
        }
    }

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    (view, sampler)
}

// --- Internal wgpu initialization ---

fn init_wgpu_internal(
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
    width: u32,
    height: u32,
    window_ptr_helper: *mut c_void, // Extra arg for tracking uniqueness
) -> bool {
    log::info!("Initializing wgpu with size {}x{}", width, height);

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface_handle = RawSurfaceHandle {
        window_handle,
        display_handle,
    };

    let surface = match unsafe {
        instance
            .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&surface_handle).unwrap())
    } {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to create surface: {:?}", e);
            return false;
        }
    };

    let adapter = match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })) {
        Ok(a) => a,
        Err(e) => {
            log::error!("Failed to find suitable adapter: {:?}", e);
            return false;
        }
    };

    let (device, queue) =
        match pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("physics_core device"),
            required_features: wgpu::Features::empty(),
            required_limits: {
                let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
                limits.max_storage_buffers_per_shader_stage = 2;
                limits.max_storage_buffer_binding_size = 65536; // 64KB
                limits.max_compute_workgroup_size_x = 256;
                limits.max_compute_workgroup_size_y = 256;
                limits.max_compute_workgroup_size_z = 64;
                limits.max_compute_invocations_per_workgroup = 256;
                limits.max_compute_workgroups_per_dimension = 65535;
                limits
            },
            ..Default::default()
        })) {
            Ok(dq) => dq,
            Err(e) => {
                log::error!("Failed to request device: {:?}", e);
                return false;
            }
        };

    let surface_caps = surface.get_capabilities(&adapter);

    // Pick a conservative, widely supported format. Some Android devices report exotic
    // formats first that gralloc cannot actually allocate for small render targets,
    // which leads to repeated 4x4 allocation failures. Prefer standard RGBA8/BGRA8
    // formats and only fall back to the first reported format if none are available.
    let preferred_formats = [
        wgpu::TextureFormat::Bgra8UnormSrgb,
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureFormat::Rgba8Unorm,
    ];

    let surface_format = preferred_formats
        .iter()
        .copied()
        .find(|f| surface_caps.formats.contains(f))
        .unwrap_or_else(|| {
            surface_caps
                .formats
                .first()
                .copied()
                .unwrap_or(wgpu::TextureFormat::Bgra8Unorm)
        });

    let max_dimension = device.limits().max_texture_dimension_2d;
    let width = width.min(max_dimension);
    let height = height.min(max_dimension);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    // Texture setup
    let (texture_view, sampler) = create_texture(&device, &queue);

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    });

    // Shader setup
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // --- Instance Data Setup ---
    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW;
    let mut instances = Vec::new();
    for y in 0..NUM_INSTANCES_PER_ROW {
        for x in 0..NUM_INSTANCES_PER_ROW {
            let position = [
                (x as f32 / NUM_INSTANCES_PER_ROW as f32) * 2.0 - 1.0 + (1.0 / NUM_INSTANCES_PER_ROW as f32),
                (y as f32 / NUM_INSTANCES_PER_ROW as f32) * 2.0 - 1.0 + (1.0 / NUM_INSTANCES_PER_ROW as f32),
            ];
            // Random-ish velocity
            let velocity = [
                (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0) * 0.1,
                (y as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0) * 0.1,
            ];
            instances.push(Instance {
                position,
                velocity,
                scale: 0.05,
                rotation: 0.0,
                uv: [0.0, 0.0],
            });
        }
    }

    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instances),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
    });

    // --- Compute Pipeline Setup ---

    let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("compute_bind_group_layout"),
    });

    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &compute_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: instance_buffer.as_entire_binding(),
        }],
        label: Some("compute_bind_group"),
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &shader,
        entry_point: Some("update_instances"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });


    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc(), Instance::desc()], // Added Instance buffer layout
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    let state = WgpuState {
        instance,
        device,
        queue,
        surface,
        config,
        render_pipeline,
        compute_pipeline,     // NEW
        vertex_buffer,
        index_buffer,
        instance_buffer,      // NEW
        diffuse_bind_group,
        compute_bind_group,   // NEW
        num_instances: NUM_INSTANCES, // NEW
        window_ptr: window_ptr_helper,
        #[cfg(target_arch = "wasm32")]
        last_render_time: web_sys::window().unwrap().performance().unwrap().now(),
        #[cfg(not(target_arch = "wasm32"))]
        last_render_time: std::time::Instant::now(),

        frame_count: 0,
        #[cfg(target_arch = "wasm32")]
        last_fps_log_time: web_sys::window().unwrap().performance().unwrap().now(),
        #[cfg(not(target_arch = "wasm32"))]
        last_fps_log_time: std::time::Instant::now(),
    };

    #[cfg(target_arch = "wasm32")]
    {
        // ... (omitted)
    }

    // Since we returned bool, we need to store state somehow.
    // The C-style init function usually returns a pointer or sets a global?
    // Wait, init_wgpu_internal returns `bool`.
    // And `wgpu_init` uses it.
    // But `wgpu_init` returns bool.
    // Wait, where is `state` stored?
    // Ah, `init_wgpu_internal` is supposed to return `bool`?
    // The original code probably returned `state` or stored it in a pointer passed as argument.
    // Let's check init_wgpu_internal signature.
    // It takes `state_ptr: *mut *mut WgpuState`.
    // I need to confirm I didn't verify the signature change.
    // But I am just replacing the body.
    
    // Removed impl WgpuState from here

    if let Ok(mut guard) = WGPU_STATE.lock() {
        guard.0 = Some(state);
    }

    INITIALIZED.store(true, Ordering::Relaxed);
    
    // Quick Integration Check (Verify compilation/linking)
    {
        use bevy_ecs::world::World;
        use rapier3d::prelude::*;
        log::info!("Verifying Physics/ECS integration...");
        let mut world = World::new();
        let _pipeline = PhysicsPipeline::new();
        let entity = world.spawn_empty().id();
        log::info!("Spawned entity: {:?}, Physics pipeline created.", entity);
    }

    true
}

fn resize_internal(width: u32, height: u32) {
    if let Ok(mut guard) = WGPU_STATE.lock() {
        if let Some(state) = guard.0.as_mut() {
            if width > 0 && height > 0 {
                let max_dimension = state.device.limits().max_texture_dimension_2d;
                let width = width.min(max_dimension);
                let height = height.min(max_dimension);

                state.config.width = width;
                state.config.height = height;
                state.surface.configure(&state.device, &state.config);
                log::info!("Resized surface to {}x{}", width, height);
            }
        }
    }
}

fn update_internal(_dt: f32) {
    if let Ok(guard) = WGPU_STATE.lock() {
        if let Some(state) = &guard.0 {
            // TODO: Update UI or simulation state
            // log::trace!("update_internal: dt={}", dt); 
        }
    }
}

fn render_internal() {
    // log::info!("render_internal called");

    // Throttling Logic (60 FPS Cap)
    if let Ok(mut guard) = WGPU_STATE.lock() {
        if let Some(state) = guard.0.as_mut() {
             #[cfg(target_arch = "wasm32")]
             {
                 let now = web_sys::window().unwrap().performance().unwrap().now();
                 let elapsed = now - state.last_render_time;
                 // 16.6ms = 1000/60. Allow slight tolerance?
                 if elapsed < 16.0 {
                     return;
                 }
                 state.last_render_time = now;
             }

             #[cfg(not(target_arch = "wasm32"))]
             {
                 let now = std::time::Instant::now();
                 let elapsed = now.duration_since(state.last_render_time);
                 if elapsed.as_millis() < 16 {
                     return;
                 }
                 state.last_render_time = now;
             }
        }
    }

    if let Ok(mut guard) = WGPU_STATE.lock() {
        if let Some(state) = guard.0.as_mut() {
            let output = match state.surface.get_current_texture() {
                Ok(o) => o,
                Err(e) => {
                    log::warn!("Failed to get current texture: {:?}", e);
                    match e {
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::OutOfMemory => {
                            log::error!("Surface lost or out of memory, resetting WGPU_STATE");
                            guard.0 = None;
                            INITIALIZED.store(false, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                    return;
                }
            };

            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // --- Compute Encoder ---
            {
                let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Compute Encoder"),
                });
                {
                    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("Compute Pass"),
                        timestamp_writes: None,
                    });
                    compute_pass.set_pipeline(&state.compute_pipeline);
                    compute_pass.set_bind_group(0, &state.compute_bind_group, &[]);
                    compute_pass.dispatch_workgroups(2, 1, 1);
                }
                state.queue.submit(std::iter::once(encoder.finish()));
            }

            // --- Render Encoder ---
            {
                let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1, 
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    render_pass.set_pipeline(&state.render_pipeline);
                    render_pass.set_bind_group(0, &state.diffuse_bind_group, &[]);
                    
                    render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));
                    
                    render_pass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    
                    render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..state.num_instances);
                }
                state.queue.submit(std::iter::once(encoder.finish()));
            }

            output.present();

            // FPS Logging
            state.frame_count += 1;
            #[cfg(target_arch = "wasm32")]
            {
                let now = web_sys::window().unwrap().performance().unwrap().now();
                let elapsed = (now - state.last_fps_log_time) / 1000.0;
                if elapsed >= 1.0 {
                     log::info!("FPS: {:.2}", state.frame_count as f64 / elapsed);
                     state.frame_count = 0;
                     state.last_fps_log_time = now;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let now = std::time::Instant::now();
                let elapsed = now.duration_since(state.last_fps_log_time).as_secs_f64();
                if elapsed >= 1.0 {
                     log::info!("FPS: {:.2}", state.frame_count as f64 / elapsed);
                     state.frame_count = 0;
                     state.last_fps_log_time = now;
                }
            }

        } else {
            // log::warn!("WGPU_STATE is None");
        }
    } else {
        log::error!("Failed to lock WGPU_STATE");
    }
}

fn shutdown_internal() {
    log::info!("Shutting down wgpu");
    if let Ok(mut guard) = WGPU_STATE.lock() {
        guard.0 = None;
    }
    INITIALIZED.store(false, Ordering::Relaxed);
}

// --- C / iOS Interface ---

#[no_mangle]
pub extern "C" fn physics_core_get_info() -> *mut c_char {
    let s = get_internal_info();
    let c_str = CString::new(s).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub(crate) extern "C" fn update_physics_internal(state: *mut WgpuState, _dt: f32) {
    let _state = unsafe { &mut *state };
    // Update physics here
}

#[no_mangle]
pub extern "C" fn physics_core_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn init_logging() {
    use std::sync::Once;
    static START: Once = Once::new();
    START.call_once(|| {
        // Configure flexi_logger:
        // - Output to stderr (for console)
        // - Output to a rotating file in "logs/" directory
        // - Default level override via RUST_LOG supported
        use flexi_logger::{Logger, FileSpec, Criterion, Naming, Cleanup, Duplicate};
        
        // Print CWD to locate logs
        if let Ok(cwd) = std::env::current_dir() {
            println!("Rust Native Library Running in: {:?}", cwd);
        }

        let _ = Logger::try_with_env_or_str("debug")
            .expect("Failed to create logger")
            .log_to_file(FileSpec::default().directory("logs").basename("physics_core"))
            .duplicate_to_stderr(Duplicate::All) // Print to console too
            .rotate(
                Criterion::Size(1024 * 1024), // 1MB
                Naming::Timestamps,
                Cleanup::KeepLogFiles(7), 
            )
            .start();
    });
}

#[no_mangle]
pub extern "C" fn wgpu_init(
    surface_handle: *mut std::ffi::c_void,
    width: i32,
    height: i32,
) -> bool {
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        init_logging();
    }

    log::debug!(
        "wgpu_init called: {:?}, {}x{}",
        surface_handle,
        width,
        height
    );

    if surface_handle.is_null() {
        log::warn!("wgpu_init: surface_handle is null, cannot initialize");
        INITIALIZED.store(true, Ordering::Relaxed);
        return true;
    }

    #[cfg(target_arch = "wasm32")]
    {
        // On WASM, wgpu_init shouldn't be called directly, we use wasm_init
        // But if it is, we just return false
        log::warn!("wgpu_init called on WASM, ignoring");
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    let (window_handle, display_handle) = {
        #[cfg(target_os = "ios")]
        let (window_handle, display_handle) = {
            use raw_window_handle::UiKitWindowHandle;
            let handle =
                UiKitWindowHandle::new(std::ptr::NonNull::new(surface_handle.cast()).unwrap());
            (
                RawWindowHandle::UiKit(handle),
                RawDisplayHandle::UiKit(raw_window_handle::UiKitDisplayHandle::new()),
            )
        };

        #[cfg(target_os = "macos")]
        let (window_handle, display_handle) = {
            use raw_window_handle::{AppKitDisplayHandle, AppKitWindowHandle};
            let handle =
                AppKitWindowHandle::new(std::ptr::NonNull::new(surface_handle.cast()).unwrap());
            (
                RawWindowHandle::AppKit(handle),
                RawDisplayHandle::AppKit(AppKitDisplayHandle::new()),
            )
        };

        #[cfg(target_os = "windows")]
        let (window_handle, display_handle) = {
            use raw_window_handle::{Win32WindowHandle, WindowsDisplayHandle};
            let mut handle = Win32WindowHandle::new(
                std::num::NonZeroIsize::new(surface_handle as isize).unwrap(),
            );
            (
                RawWindowHandle::Win32(handle),
                RawDisplayHandle::Windows(WindowsDisplayHandle::new()),
            )
        };

        #[cfg(all(
            unix,
            not(any(target_os = "ios", target_os = "macos", target_os = "android"))
        ))]
        let (window_handle, display_handle) = {
            use raw_window_handle::{XlibDisplayHandle, XlibWindowHandle};
            let handle = XlibWindowHandle::new(surface_handle as u64);
            (
                RawWindowHandle::Xlib(handle),
                RawDisplayHandle::Xlib(XlibDisplayHandle::new(None, 0)),
            )
        };

        #[cfg(target_os = "android")]
        let (window_handle, display_handle) = {
            use raw_window_handle::{AndroidDisplayHandle, AndroidNdkWindowHandle};
            let handle =
                AndroidNdkWindowHandle::new(std::ptr::NonNull::new(surface_handle.cast()).unwrap());
            (
                RawWindowHandle::AndroidNdk(handle),
                RawDisplayHandle::Android(AndroidDisplayHandle::new()),
            )
        };

        (window_handle, display_handle)
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        init_wgpu_internal(
            window_handle,
            display_handle,
            width as u32,
            height as u32,
            surface_handle,
        )
    }

    #[cfg(target_arch = "wasm32")]
    false
}

#[no_mangle]
pub extern "C" fn wgpu_update(delta_time: f32) {
    if !INITIALIZED.load(Ordering::Relaxed) {
        return;
    }
    // TODO: Update game logic
    log::trace!("wgpu_update: dt={}", delta_time);
    update_internal(delta_time);
}

#[no_mangle]
pub extern "C" fn wgpu_render() {
    if !INITIALIZED.load(Ordering::Relaxed) {
        return;
    }
    render_internal();
}

#[no_mangle]
pub extern "C" fn wgpu_resize(width: i32, height: i32) {
    if !INITIALIZED.load(Ordering::Relaxed) {
        return;
    }
    log::info!("wgpu_resize: {}x{}", width, height);
    resize_internal(width as u32, height as u32);
}

#[no_mangle]
pub extern "C" fn wgpu_shutdown() {
    log::info!("wgpu_shutdown called");
    shutdown_internal();
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

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeInit(
    _env: JNIEnv,
    _class: JClass,
    surface_handle: jlong,
    width: jint,
    height: jint,
) -> jboolean {
    wgpu_init(
        surface_handle as *mut std::ffi::c_void,
        width as i32,
        height as i32,
    ) as jboolean
}

#[cfg(feature = "jni_support")]
#[no_mangle]
pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeUpdate(
    _env: JNIEnv,
    _class: JClass,
    delta_time: jfloat,
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
    width: jint,
    height: jint,
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

// --- Wasm Interface ---

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_get_info() -> String {
    get_internal_info()
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub async fn wasm_init(canvas_id: &str, width: u32, height: u32) -> bool {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _ = console_log::init_with_level(log::Level::Info);
    log::info!(
        "wasm_init called: canvas={}, {}x{}",
        canvas_id,
        width,
        height
    );

    // For WASM, we use web_sys to get the canvas element
    use wasm_bindgen::JsCast;

    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            log::error!("No window available");
            return false;
        }
    };

    let document = match window.document() {
        Some(d) => d,
        None => {
            log::error!("No document available");
            return false;
        }
    };

    let canvas = match document.get_element_by_id(canvas_id) {
        Some(c) => c,
        None => {
            log::error!("Canvas element '{}' not found", canvas_id);
            return false;
        }
    };

    let canvas: web_sys::HtmlCanvasElement = match canvas.dyn_into() {
        Ok(c) => c,
        Err(_) => {
            log::error!("Element '{}' is not a canvas", canvas_id);
            return false;
        }
    };

    // Set initial scale factor - REMOVED wgpu_set_scale_factor calls
    // Attach Input Listeners - REMOVED

    // Apply debug style to the EXISTING canvas to verify we have it
    // We wrap these in a block and ignore errors just in case

    log::info!("Acquired existing canvas: {}", canvas_id);

    // Helper function to try initializing a specific backend
    let init_backend = |backend: wgpu::Backends, canvas: web_sys::HtmlCanvasElement| async move {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: backend,
            ..Default::default()
        });

        let surface = match instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to create surface: {:?}", e)),
        };

        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
        {
            Ok(a) => a,
            Err(_) => return Err("Failed to find suitable adapter".to_string()),
        };

        log::info!("Adapter backend: {:?}", adapter.get_info().backend);
        log::info!("Adapter limits: {:?}", adapter.limits());

        let requested_limits = if backend == wgpu::Backends::BROWSER_WEBGPU {
            // For WebGPU, trust the adapter to handle its own limits
            adapter.limits()
        } else {
            // For WebGL, use safe downlevel defaults but try to bump texture size
            let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
            let adapter_limits = adapter.limits();
            limits.max_texture_dimension_2d = adapter_limits.max_texture_dimension_2d;
            limits
        };

        log::info!("Requesting limits: {:?}", requested_limits);

        let (device, queue) = match adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("physics_core device"),
                required_features: wgpu::Features::empty(),
                required_limits: requested_limits,
                ..Default::default()
            })
            .await
        {
            Ok(dq) => dq,
            Err(e) => return Err(format!("Failed to request device: {:?}", e)),
        };

        Ok((instance, surface, adapter, device, queue))
    };

    // Try WebGPU first
    let result = match init_backend(wgpu::Backends::BROWSER_WEBGPU, canvas.clone()).await {
        Ok(res) => Ok(res),
        Err(e) => {
            log::warn!(
                "WebGPU initialization failed: {}. Replacing canvas and falling back to WebGL...",
                e
            );

            // Replace the canvas to clear any tainted context
            let parent = canvas.parent_node().unwrap();
            let new_canvas_node = canvas.clone_node().unwrap();
            parent.replace_child(&new_canvas_node, &canvas).unwrap();

            let new_canvas: web_sys::HtmlCanvasElement = new_canvas_node.dyn_into().unwrap();

            init_backend(wgpu::Backends::GL, new_canvas).await
        }
    };

    let (instance, surface, adapter, device, queue) = match result {
        Ok(res) => res,
        Err(e) => {
            log::error!("Final initialization failed: {}", e);
            return false;
        }
    };

    log::info!("Device acquired. getting surface caps...");

    let surface_caps = surface.get_capabilities(&adapter);
    log::info!(
        "Surface caps acquired. Alpha Modes: {:?}",
        surface_caps.alpha_modes
    );

    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    log::info!("Selected surface format: {:?}", surface_format);

    let max_dimension = device.limits().max_texture_dimension_2d;

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: width.min(max_dimension),
        height: height.min(max_dimension),
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    log::info!("Surface config created: {}x{}", config.width, config.height);

    surface.configure(&device, &config);
    log::info!("Surface configured");
    let (texture_view, sampler) = create_texture(&device, &queue);

    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("diffuse_bind_group"),
    });

    // Shader setup
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc(), Instance::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    // --- Instance Data Setup ---
    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW;
    let mut instances = Vec::new();
    for y in 0..NUM_INSTANCES_PER_ROW {
        for x in 0..NUM_INSTANCES_PER_ROW {
            let position = [
                (x as f32 / NUM_INSTANCES_PER_ROW as f32) * 2.0 - 1.0 + (1.0 / NUM_INSTANCES_PER_ROW as f32),
                (y as f32 / NUM_INSTANCES_PER_ROW as f32) * 2.0 - 1.0 + (1.0 / NUM_INSTANCES_PER_ROW as f32),
            ];
            // Random-ish velocity
            let velocity = [
                (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0) * 0.1,
                (y as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0) * 0.1,
            ];
            instances.push(Instance {
                position,
                velocity,
                scale: 0.05,
                rotation: 0.0,
                uv: [0.0, 0.0],
            });
        }
    }

    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instances),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
    });

    // --- Compute Pipeline Setup ---
    let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("compute_bind_group_layout"),
    });

    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &compute_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: instance_buffer.as_entire_binding(),
        }],
        label: Some("compute_bind_group"),
    });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &shader,
        entry_point: Some("update_instances"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let state = WgpuState {
        instance,
        device,
        queue,
        surface,
        config,
        render_pipeline,
        vertex_buffer,
        index_buffer,
        instance_buffer,      // NEW
        diffuse_bind_group,
        compute_bind_group,   // NEW
        compute_pipeline,     // NEW
        num_instances: NUM_INSTANCES, // NEW
        window_ptr: std::ptr::null_mut(),
        #[cfg(target_arch = "wasm32")]
        last_render_time: web_sys::window().unwrap().performance().unwrap().now(),
        #[cfg(not(target_arch = "wasm32"))]
        last_render_time: std::time::Instant::now(),

        frame_count: 0,
        #[cfg(target_arch = "wasm32")]
        last_fps_log_time: web_sys::window().unwrap().performance().unwrap().now(),
        #[cfg(not(target_arch = "wasm32"))]
        last_fps_log_time: std::time::Instant::now(),
    };

    if let Ok(mut guard) = WGPU_STATE.lock() {
        guard.0 = Some(state);
    }

    match INITIALIZED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => log::info!("WASM wgpu initialized successfully"),
        Err(_) => log::warn!("WASM wgpu already initialized"),
    }

    true
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_update(delta_time: f32) {
    wgpu_update(delta_time);
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_render() {
    // log::info!("wasm_render called");
    render_internal();
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_resize(width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }

    if let Ok(mut guard) = WGPU_STATE.lock() {
        if let Some(state) = guard.0.as_mut() {
            let max_dimension = state.device.limits().max_texture_dimension_2d;
            let clamped_width = width.min(max_dimension);
            let clamped_height = height.min(max_dimension);

            log::info!(
                "wasm_resize: {}x{} -> clamped {}x{}",
                width,
                height,
                clamped_width,
                clamped_height
            );

            state.config.width = clamped_width;
            state.config.height = clamped_height;
            state.surface.configure(&state.device, &state.config);
        }
    }
}

#[cfg(feature = "wasm_support")]
#[wasm_bindgen]
pub fn wasm_shutdown() {
    wgpu_shutdown();
}

// --- Winit Standalone App (for JVM Debugging) ---

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub fn start_winit_app() {
    use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

    use winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new().unwrap();
    let mut last_frame_time = std::time::Instant::now();

    let window = std::sync::Arc::new(
        WindowBuilder::new()
            .with_title("PhysicsFX (Rust Winit)")
            .with_inner_size(winit::dpi::PhysicalSize::new(1600, 900))
            .build(&event_loop)
            .unwrap(),
    );

    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        init_logging();
    }

    let width = window.inner_size().width;

    let height = window.inner_size().height;

    let window_handle = window.window_handle().unwrap().as_raw();

    let display_handle = window.display_handle().unwrap().as_raw();

    // Note: init_wgpu_internal expects rwh::RawWindowHandle, etc.

    if !init_wgpu_internal(
        window_handle,
        display_handle,
        width,
        height,
        std::ptr::null_mut(),
    ) {
        // Pass null for helper if not needed or not available easily
        log::error!("Failed to initialize wgpu");

        return;
    }

    event_loop
        .run(|event, target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => target.exit(),

                WindowEvent::Resized(size) => {
                    let width = size.width;

                    let height = size.height;

                    if width > 0 && height > 0 {
                        resize_internal(width, height);

                        window.request_redraw();
                    }
                }

                WindowEvent::RedrawRequested => {
                    let now = std::time::Instant::now();
                    let dt = now.duration_since(last_frame_time).as_secs_f32();
                    last_frame_time = now;
                    
                    update_internal(dt);
                    render_internal();

                    window.request_redraw();
                    // Sleep to prevent CPU burn if redraw is fast
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }

                _ => (),
            },

            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        })
        .unwrap();
}

#[cfg(feature = "jni_support")]
#[no_mangle]

pub extern "system" fn Java_app_kamkash_physicsfx_JvmWgpuGameLoop_nativeStartWinitApp(
    _env: JNIEnv,

    _class: JClass,
) {
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    start_winit_app();
}

#[cfg(target_os = "android")]
#[no_mangle]
// #[::android_activity::android_main]
pub extern "C" fn android_main(app: AndroidApp) {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("PhysicsFX"),
    );

    let mut quit = false;
    let mut suspended = false;
    let mut redraw_requested = true;
    let mut last_frame_time = std::time::Instant::now();

    while !quit {
        if let Ok(mut iter) = app.input_events_iter() {
            while iter.next(|event| {
                match event {
                    InputEvent::MotionEvent(motion) => {
                        if motion.action() == MotionAction::Up {
                            log::info!("Touch up event");
                        }
                    }
                    _ => {}
                }
                android_activity::InputStatus::Handled
            }) {}
        }

        app.poll_events(
            Some(std::time::Duration::from_millis(16)),
            |event| match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    log::info!("MainEvent::Destroy");
                    shutdown_internal();
                    quit = true;
                }

                // PollEvent::Main(MainEvent::TermWindow { .. }) => { // Error: variant not found
                //    log::info!("MainEvent::TermWindow");
                //    shutdown_internal();
                // }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    log::info!("MainEvent::TerminateWindow");
                    shutdown_internal();
                }

                PollEvent::Main(MainEvent::Pause) => {
                    log::info!("MainEvent::Pause");
                    suspended = true;
                }

                PollEvent::Main(MainEvent::Resume { .. }) => {
                    log::info!("MainEvent::Resume");
                    suspended = false;
                }

                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    log::info!("MainEvent::InitWindow");
                    suspended = false; // Ensure we are not suspended if we get a new window
                    if let Some(window) = app.native_window() {
                        let window_ptr = window.ptr().as_ptr();

                        unsafe {
                             ANativeWindow_acquire(window_ptr as *mut c_void);
                        }

                        // Fix 3: Wrap pointer in NonNull for NDK
                        let non_null_ptr = NonNull::new(window_ptr).unwrap();

                        let native_window =
                            unsafe { ndk::native_window::NativeWindow::from_ptr(non_null_ptr) };
                        
                        let width = native_window.width();
                        let height = native_window.height();

                        // Fix 4: Cast to c_void for raw-window-handle
                        // window_ptr is *mut ANativeWindow, we need NonNull<c_void>
                        let mut window_handle =
                            AndroidNdkWindowHandle::new(non_null_ptr.cast::<c_void>());

                        let display_handle = AndroidDisplayHandle::new();

                        // Call your internal init (make sure signature matches)
                        if !init_wgpu_internal(
                            RawWindowHandle::AndroidNdk(window_handle),
                            RawDisplayHandle::Android(display_handle),
                            width as u32,
                            height as u32,
                            window_ptr as *mut c_void,
                        ) {
                            log::error!("Failed to initialize wgpu");
                            // quit = true; // Don't quit, try to recover or wait for next window
                        }
                    }
                }

                PollEvent::Main(MainEvent::WindowResized { .. }) => {
                    log::info!("MainEvent::WindowResized ");
                    if let Some(window) = app.native_window() {
                        let window_ptr = window.ptr().as_ptr();
                        
                        // Fix refcount issue
                        unsafe {
                            ANativeWindow_acquire(window_ptr as *mut c_void);
                        }

                        let non_null_ptr = NonNull::new(window_ptr).unwrap();

                        let native_window =
                            unsafe { ndk::native_window::NativeWindow::from_ptr(non_null_ptr) };

                        let width = native_window.width();
                        let height = native_window.height();

                        // Check if window pointer changed
                        let mut recreate_needed = false;
                        if let Ok(guard) = WGPU_STATE.lock() {
                            if let Some(state) = &guard.0 {
                                if state.window_ptr != window_ptr as *mut c_void {
                                    log::warn!("WindowResized: Window pointer changed! Recreating surface.");
                                    recreate_needed = true;
                                }
                            }
                        }

                        if recreate_needed {
                             // Re-run init logic
                             log::info!("Re-initializing WGPU due to window change");
                             // logic copied/refactored from InitWindow
                             let non_null_ptr = NonNull::new(window_ptr).unwrap();
                             let window_handle = AndroidNdkWindowHandle::new(non_null_ptr.cast::<c_void>());
                             let display_handle = AndroidDisplayHandle::new();
                             init_wgpu_internal(
                                RawWindowHandle::AndroidNdk(window_handle),
                                RawDisplayHandle::Android(display_handle),
                                width as u32,
                                height as u32,
                                window_ptr as *mut c_void,
                            );
                        } else {
                            resize_internal(width as u32, height as u32);
                        }
                    }
                }

                PollEvent::Main(MainEvent::RedrawNeeded { .. }) => {
                    redraw_requested = true;
                }

                _ => {}
            },
        );

        if !suspended {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            update_internal(dt);
            render_internal();
            // redraw_requested = false; // logic removed
            // Sleep to prevent hot loop
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

