Yes! There are several excellent Rust graphics libraries with features comparable to Three.js. Here are the main options:

## 1. **Bevy** (Recommended - Most Three.js-like)

Bevy is a data-driven game engine with a scene graph model similar to Three.js.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a cube (like THREE.BoxGeometry + THREE.Mesh)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Add camera (like THREE.PerspectiveCamera)
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add light (like THREE.DirectionalLight)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        ..default()
    });
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Handle<Mesh>>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds());
        transform.rotate_x(time.delta_seconds() * 0.5);
    }
}
```

**Features:**

- ✅ Scene graph with entities and components
- ✅ PBR materials (like MeshStandardMaterial)
- ✅ Built-in cameras, lights, meshes
- ✅ Animation system
- ✅ Asset loading (GLTF, textures, etc.)
- ✅ Cross-platform (Desktop, Web via WASM, Mobile)
- ✅ Built on wgpu (WebGPU standard)

**Comparison to Three.js:**

- Similar high-level API
- ECS (Entity Component System) instead of scene graph
- More performant due to Rust + ECS
- Steeper learning curve but more powerful

## 2. **three-d**

Directly inspired by Three.js with very similar API:

```rust
use three_d::*;

fn main() {
    let window = Window::new(WindowSettings {
        title: "Three-d Example".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    }).unwrap();

    let context = window.gl();

    // Camera (like THREE.PerspectiveCamera)
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    // Create a cube mesh
    let mut cpu_mesh = CpuMesh::cube();
    cpu_mesh.transform(&Mat4::from_scale(2.0)).unwrap();
    
    let mut model = Gm::new(
        Mesh::new(&context, &cpu_mesh),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new(0, 255, 0, 255),
                ..Default::default()
            },
        ),
    );

    // Lighting
    let ambient = AmbientLight::new(&context, 0.3, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(0.0, -1.0, 0.0));

    // Render loop
    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        
        // Rotate the cube
        model.set_transformation(Mat4::from_angle_y(radians(
            (frame_input.accumulated_time * 0.001) as f32
        )));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, &model, &[&ambient, &directional]);

        FrameOutput::default()
    });
}
```

**Features:**

- ✅ API very similar to Three.js
- ✅ PBR rendering
- ✅ GLTF loading
- ✅ Post-processing effects
- ✅ Cross-platform (Desktop + WASM)
- ✅ Built on wgpu

**Best for:** Developers familiar with Three.js wanting to switch to Rust

## 3. **wgpu + rend3** (Lower-level but powerful)

```rust
use rend3::*;
use rend3_routine::*;

// Similar to using raw WebGPU with helpers
// More control, more boilerplate
```

**Features:**

- ✅ Modern rendering architecture
- ✅ PBR, skinning, animations
- ✅ Scene graph
- ✅ Very performant

**Best for:** Advanced users who need fine control

## 4. **Fyrox** (Full Game Engine)

```rust
use fyrox::prelude::*;

// Similar to Unity/Unreal but in Rust
// Full-featured editor
```

**Features:**

- ✅ Complete game engine with editor
- ✅ Scene graph, materials, animations
- ✅ Physics integration
- ✅ UI system
- ✅ Cross-platform

**Best for:** Complete game development

## Feature Comparison Table

| Feature | Three.js | Bevy | three-d | wgpu (raw) |
|---------|----------|------|---------|------------|
| Scene Graph | ✅ | ✅ (ECS) | ✅ | ❌ (DIY) |
| PBR Materials | ✅ | ✅ | ✅ | 🔧 (manual) |
| GLTF Loading | ✅ | ✅ | ✅ | 🔧 (manual) |
| Animations | ✅ | ✅ | ✅ | 🔧 (manual) |
| Post-processing | ✅ | ✅ | ✅ | 🔧 (manual) |
| Shadows | ✅ | ✅ | ✅ | 🔧 (manual) |
| Desktop | ❌ (browser) | ✅ | ✅ | ✅ |
| Mobile | ✅ (browser) | ✅ | ⚠️ (limited) | ✅ |
| Web (WASM) | ✅ | ✅ | ✅ | ✅ |
| Learning Curve | Easy | Medium | Easy | Hard |
| Performance | Good | Excellent | Excellent | Excellent |

## Deployment Comparison

### Three.js (Browser/Deno)

```bash
# Desktop: Deno + WebGPU
deno compile --unstable-webgpu app.ts

# Web: Just deploy
npm run build
```

### Bevy (Rust)

```bash
# Desktop: Single binary
cargo build --release
# Output: ~10-50MB executable

# Web: WASM
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/game.wasm

# Mobile: 
cargo build --release --target aarch64-linux-android
cargo build --release --target aarch64-apple-ios
```

## Real-World Example: Three.js vs Bevy

**Three.js:**

```javascript
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, width/height, 0.1, 1000);
const renderer = new THREE.WebGPURenderer();

const geometry = new THREE.BoxGeometry();
const material = new THREE.MeshStandardMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

const light = new THREE.DirectionalLight(0xffffff, 1);
scene.add(light);

function animate() {
  cube.rotation.x += 0.01;
  cube.rotation.y += 0.01;
  renderer.render(scene, camera);
  requestAnimationFrame(animate);
}
```

**Bevy (equivalent):**

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
        ..default()
    });
    
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
    
    commands.spawn(DirectionalLightBundle::default());
}

fn animate(mut query: Query<&mut Transform, With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_x(0.01);
        transform.rotate_y(0.01);
    }
}
```

## My Recommendation

**If you want the closest Three.js equivalent:** Use **three-d** - nearly identical API, easy transition

**If you want the best performance and features:** Use **Bevy** - modern architecture, excellent ecosystem, very active development

**If you need full mobile support today:** Use **Bevy** - mature iOS/Android support

**If you prefer staying with JavaScript/TypeScript:** Stick with Deno + Three.js for desktop, React Native for mobile

The Rust options will give you **better performance**, **smaller binaries**, **true native deployment**, and **cross-platform support including mobile** - but with a steeper learning
curve. Bevy in particular has momentum and could become the "Unity of Rust" in terms of popularity and features.

Great question! Let's break down what **three-d** adds on top of your existing `wgpu + bevy_ecs + rapier3d` stack to see if that 2-5MB is worth it:

## What You Already Have (wgpu + bevy_ecs + rapier3d)

**wgpu**: Low-level GPU API (like WebGPU)

- Raw rendering commands
- Shader compilation
- GPU resource management
- ❌ No high-level graphics features

**bevy_ecs**: Data management

- Entity component system
- ✅ Game logic organization
- ❌ No rendering helpers

**rapier3d**: Physics

- ✅ Collision detection, rigid bodies
- ❌ No rendering/visuals

**What you're missing**: Everything between "GPU commands" and "pixels on screen"

## What three-d Adds (Worth 2-5MB?)

### 1. **Material System** (~500KB)

**Without three-d:**

```rust
// You write raw wgpu shaders and bind groups
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    source: wgpu::ShaderSource::Wgsl(r#"
        @vertex
        fn vs_main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
            return vec4<f32>(pos, 1.0);
        }
        
        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red
        }
    "#.into()),
});

// Manual bind group layout for textures, uniforms, etc.
let bind_group_layout = device.create_bind_group_layout(/* ... 50+ lines */);
// Manual pipeline creation
// Manual uniform buffer management
// etc...
```

**With three-d:**

```rust
// PBR material with metallic/roughness workflow
let material = PhysicalMaterial::new(
    &context,
    &CpuMaterial {
        albedo: Srgba::new(255, 0, 0, 255),
        metallic: 0.5,
        roughness: 0.3,
        albedo_texture: Some(texture),
        normal_texture: Some(normal_map),
        occlusion_metallic_roughness_texture: Some(orm_map),
        ..Default::default()
    },
);
```

**Features you get:**

- ✅ **PBR (Physically Based Rendering)** - Industry-standard shading
- ✅ **Texture support** - Albedo, normal, metallic, roughness, AO, emissive
- ✅ **Normal mapping** - Surface detail without geometry
- ✅ **Parallax mapping** - Advanced surface detail
- ✅ **Transparency** - Alpha blending, alpha cutout
- ✅ **Two-sided materials** - No backface culling issues
- ✅ **Pre-built shaders** - No manual WGSL writing

### 2. **Lighting System** (~300KB)

**Without three-d:**

```rust
// Manual light uniform buffers
struct LightUniform {
    position: [f32; 3],
    color: [f32; 3],
    intensity: f32,
    // ... write shader code for each light type
}
// Implement shadow mapping manually (500+ lines)
// Implement light attenuation manually
// etc.
```

**With three-d:**

```rust
let lights: Vec<Box<dyn Light>> = vec![
    Box::new(DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, -1.0))),
    Box::new(PointLight::new(&context, 1.0, Srgba::new(255, 0, 0, 255), &vec3(2.0, 2.0, 2.0), 
        Attenuation::default())),
    Box::new(SpotLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 5.0, 0.0), 
        &vec3(0.0, -1.0, 0.0), 25.0, 30.0, Attenuation::default())),
    Box::new(AmbientLight::new(&context, 0.2, Srgba::WHITE)),
];
```

**Features you get:**

- ✅ **Directional lights** - Sun/moon lighting
- ✅ **Point lights** - Omni-directional (light bulbs)
- ✅ **Spot lights** - Cone-shaped (flashlights)
- ✅ **Ambient light** - Base illumination
- ✅ **Attenuation** - Realistic light falloff
- ✅ **Shadow mapping** - Built-in shadow rendering
- ✅ **Multiple lights** - Automatic multi-light support

### 3. **Camera System** (~200KB)

**Without three-d:**

```rust
// Manual projection matrix
fn perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    // 20+ lines of math
}

// Manual view matrix
fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    // 15+ lines of math
}

// Manual frustum culling
// Manual viewport management
```

**With three-d:**

```rust
let mut camera = Camera::new_perspective(
    viewport,
    vec3(5.0, 5.0, 5.0),  // eye
    vec3(0.0, 0.0, 0.0),  // target
    vec3(0.0, 1.0, 0.0),  // up
    degrees(45.0),         // fov
    0.1,                   // near
    1000.0                 // far
);

// Built-in camera controls
camera.rotate_around_with_fixed_up(&target, delta_x, delta_y);
camera.zoom_towards(&target, zoom_delta, 0.1, 100.0);
```

**Features you get:**

- ✅ **Perspective camera** - Standard 3D camera
- ✅ **Orthographic camera** - For 2D/isometric
- ✅ **Camera controls** - Orbit, pan, zoom helpers
- ✅ **Viewport management** - Automatic aspect ratio
- ✅ **Frustum culling** - Automatic visibility testing
- ✅ **View/projection matrices** - Pre-calculated

### 4. **Geometry Primitives** (~400KB)

**Without three-d:**

```rust
// Manual vertex/index buffer creation for basic shapes
fn create_cube_mesh() -> (Vec<Vertex>, Vec<u32>) {
    // 100+ lines to generate cube vertices, normals, UVs, indices
}
fn create_sphere_mesh(subdivisions: u32) -> (Vec<Vertex>, Vec<u32>) {
    // 200+ lines of sphere tessellation math
}
```

**With three-d:**

```rust
// One-liners for common shapes
let cube = CpuMesh::cube();
let sphere = CpuMesh::sphere(32);
let cylinder = CpuMesh::cylinder(16);
let cone = CpuMesh::cone(16);
let torus = CpuMesh::torus(32, 16, 1.0, 0.3);
let plane = CpuMesh::square();
let arrow = CpuMesh::arrow(1.0, 0.1, 16);
```

**Features you get:**

- ✅ **Cube, sphere, cylinder, cone** - Basic primitives
- ✅ **Torus, plane, arrow** - Utility shapes
- ✅ **Auto-generated normals** - Proper lighting
- ✅ **Auto-generated UVs** - Texture mapping
- ✅ **Auto-generated tangents** - Normal mapping support

### 5. **Asset Loading** (~800KB)

**Without three-d:**

```rust
// Manual GLTF parsing (or use gltf crate + 500+ lines of integration)
use gltf;
// Parse JSON
// Extract buffers
// Build mesh data structures
// Load textures
// Apply transforms
// Set up materials
// etc... (easily 1000+ lines)
```

**With three-d:**

```rust
// Load complete 3D models
let mut cpu_model: CpuModel = three_d_asset::io::load("model.gltf")?;

// Automatically handles:
// - Multiple meshes
// - Materials with textures
// - Scene hierarchy
// - Animations
```

**Features you get:**

- ✅ **GLTF/GLB loading** - Industry standard format
- ✅ **OBJ loading** - Legacy format support
- ✅ **Texture loading** - PNG, JPG, HDR
- ✅ **Material parsing** - PBR material extraction
- ✅ **Animation data** - Skeletal animation support
- ✅ **Scene hierarchy** - Parent/child transforms

### 6. **Post-Processing Effects** (~600KB)

**Without three-d:**

```rust
// Manual render target creation
// Manual shader passes
// Manual effect composition
// Implement each effect from scratch (100+ lines each)
```

**With three-d:**

```rust
use three_d::renderer::*;

// Built-in effects
let mut fog_effect = FogEffect { /* ... */ };
let mut fxaa = FxaaEffect::new(&context);
let mut bloom = BloomEffect::new(&context);

// Apply to render
screen()
    .apply_screen_effect(
        &fxaa,
        &camera,
        &[&model],
        &lights,
    );
```

**Features you get:**

- ✅ **FXAA/MSAA** - Anti-aliasing
- ✅ **Bloom** - Glow effect
- ✅ **Fog** - Atmospheric effects
- ✅ **Depth of field** - Camera focus
- ✅ **Render to texture** - Off-screen rendering
- ✅ **Effect composition** - Chain multiple effects

### 7. **Render Pipeline Helpers** (~400KB)

**Without three-d:**

```rust
// Manual depth buffer setup
// Manual render pass configuration
// Manual clear operations
// Manual render state management
```

**With three-d:**

```rust
// High-level render operations
screen()
    .clear(ClearState::color_and_depth(0.2, 0.3, 0.4, 1.0, 1.0))
    .render(&camera, &objects, &lights)
    .write(|| {
        // Custom render pass
    });

// Automatic depth testing, blending, culling
```

**Features you get:**

- ✅ **Automatic render state** - Depth, blending, culling
- ✅ **Multi-pass rendering** - Deferred rendering support
- ✅ **Instanced rendering** - Draw many objects efficiently
- ✅ **Render queuing** - Automatic draw call batching
- ✅ **Material sorting** - Minimize state changes

### 8. **Debug Rendering** (~200KB)

```rust
// Debug visualization without extra code
let axes = Axes::new(&context, 0.01, 1.0);
let bounding_box = BoundingBox::new_with_positions(/* ... */);
let lines = Lines::new(&context, &cpu_lines);
```

**Features you get:**

- ✅ **Axes gizmo** - World orientation
- ✅ **Bounding boxes** - Volume visualization
- ✅ **Lines/wireframe** - Debug shapes
- ✅ **Normals visualization** - Surface debugging

### 9. **GUI Integration** (~300KB, optional)

```rust
use three_d::egui_gui::*;

// Immediate mode GUI
let mut gui = GUI::new(&context);

gui.update(|gui_context| {
    egui::Window::new("Settings").show(gui_context, |ui| {
        ui.checkbox(&mut wireframe, "Wireframe");
        ui.slider(f32, "Roughness", 0.0..=1.0);
    });
});
```

**Features you get:**

- ✅ **egui integration** - Immediate mode GUI
- ✅ **Debug panels** - Inspector windows
- ✅ **Controls** - Sliders, buttons, etc.

## What You DON'T Get (Still Need to Implement)

❌ **Animation system** - three-d loads animation data but doesn't play it  
❌ **Skeletal animation** - No bone/skinning system  
❌ **Particle systems** - No built-in particles  
❌ **Advanced shadows** - Only basic shadow mapping  
❌ **Deferred rendering** - Mostly forward rendering  
❌ **Global illumination** - No GI/raytracing  
❌ **LOD system** - No level-of-detail  
❌ **Terrain rendering** - No heightmap support  

## Code Comparison: With vs Without three-d

**Rendering a textured, lit cube:**

### Without three-d (~500+ lines)

```rust
// Create shader
// Create vertex buffer
// Create index buffer
// Create texture
// Create sampler
// Create bind group layout
// Create pipeline layout
// Create render pipeline
// Create uniform buffers for:
//   - Model matrix
//   - View matrix
//   - Projection matrix
//   - Light data
// Write shader code for:
//   - Vertex transformation
//   - Normal transformation
//   - Lighting calculations
//   - Texture sampling
// Manual render loop:
//   - Update uniforms
//   - Bind pipeline
//   - Bind buffers
//   - Bind textures
//   - Draw call
```

### With three-d (~15 lines)

```rust
let cube = Gm::new(
    Mesh::new(&context, &CpuMesh::cube()),
    PhysicalMaterial::new_opaque(&context, &CpuMaterial {
        albedo_texture: Some(texture),
        ..Default::default()
    }),
);

let light = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -1.0, 0.0));

screen()
    .clear(ClearState::default())
    .render(&camera, &[&cube], &[&light]);
```

## Binary Size Breakdown

| Feature | Size | Worth It? |
|---------|------|-----------|
| Material system | ~500KB | ✅ YES - Saves 1000+ lines |
| Lighting | ~300KB | ✅ YES - Saves 500+ lines |
| Camera | ~200KB | ⚠️ MAYBE - Easy to implement yourself |
| Primitives | ~400KB | ⚠️ MAYBE - Only needed once |
| Asset loading | ~800KB | ✅ YES - GLTF is complex |
| Post-processing | ~600KB | ⚠️ DEPENDS - If you need effects |
| Render helpers | ~400KB | ✅ YES - Huge boilerplate savings |
| Debug rendering | ~200KB | ⚠️ NICE TO HAVE |
| GUI (optional) | ~300KB | ⚠️ DEPENDS - Use if needed |

**Total: ~2.5-5MB depending on features enabled**

## My Recommendation

### ✅ **Add three-d if:**

- You need **PBR materials** (metallic/roughness workflow)
- You're loading **GLTF models** with complex materials
- You want **multiple light types** (point, spot, directional)
- You need **post-processing** (bloom, FXAA, etc.)
- You want to **move fast** (prototype quickly)
- **2-5MB is acceptable** for your platform

### ❌ **Skip three-d if:**

- You only render **simple shapes** (lines, basic colors)
- Your materials are **custom** (stylized, non-PBR)
- **Binary size is critical** (embedded systems, web)
- You have **custom rendering pipeline** needs
- You're willing to **write shaders manually**

### 🔧 **Alternative: Cherry-pick features**

You can use parts of three-d without the full dependency:

```rust
// Just use the mesh primitives
use three_d_asset::geometry::*;
let cube_data = CpuMesh::cube();

// Just use the material system
use three_d::renderer::material::*;

// Just use asset loading
use three_d_asset::io::*;
```

This lets you get specific features with less binary bloat.

## Bottom Line

**three-d saves you from writing ~3,000-5,000 lines of rendering code** at the cost of 2-5MB. For most games/apps, that's a great trade-off. But if you only need basic rendering, raw wgpu might be enough.

**For your KMP app**: Since you're already shipping to mobile (Android/iOS), 2-5MB is negligible. I'd **recommend adding three-d** to avoid reimplementing PBR materials and GLTF loading.

No, **three-d does NOT include** a full animation system, sprites, or sprite sheet support. Let me break down what it has and what's missing:

## What three-d HAS (Animation-related)

### 1. **Animation Data Loading Only**

```rust
use three_d_asset::io::load;

// Loads GLTF with animation data
let mut loaded: CpuModel = load("animated_model.gltf")?;

// You get the raw animation data:
for animation in &loaded.animations {
    println!("Animation: {}", animation.name);
    // But three-d doesn't play/interpolate it for you!
}
```

**What you get:**

- ✅ Loads animation keyframes from GLTF
- ✅ Parses animation channels (translation, rotation, scale)
- ✅ Stores bone/joint data

**What you DON'T get:**

- ❌ Animation playback system
- ❌ Keyframe interpolation
- ❌ Bone transformations
- ❌ Skinning/skeletal animation
- ❌ Animation blending
- ❌ Animation state machine

You'd need to implement playback yourself:

```rust
// You have to write this yourself
fn update_animation(
    animation: &Animation,
    time: f32,
    skeleton: &mut Skeleton,
) {
    // Interpolate between keyframes
    // Update bone transforms
    // Apply to mesh vertices
    // etc... (200+ lines)
}
```

### 2. **Simple Transform Animation (Manual)**

```rust
// You can animate by manually updating transforms
let mut rotation = 0.0;

loop {
    rotation += delta_time;
    
    model.set_transformation(
        Mat4::from_rotation_y(rotation)
    );
    
    render();
}
```

This is just manual property animation, not a real animation system.

## What three-d DOESN'T HAVE

### ❌ **No Sprite System**

three-d is a **3D-focused** library. It has no built-in 2D sprite support.

**What's missing:**

- No sprite rendering
- No texture atlases
- No sprite batching
- No 2D camera helpers
- No sprite layers/sorting

**Workaround (very manual):**

```rust
// You'd have to fake sprites with textured quads
let sprite = Gm::new(
    Mesh::new(&context, &CpuMesh::square()),
    ColorMaterial {
        texture: Some(texture),
        ..Default::default()
    },
);

// Position in 3D space like a billboard
sprite.set_transformation(
    Mat4::from_translation(vec3(x, y, 0.0))
        * Mat4::from_scale(width)
);

// Use orthographic camera for 2D feel
let camera = Camera::new_orthographic(
    viewport,
    vec3(0.0, 0.0, 5.0),
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    10.0,
    0.1,
    100.0,
);
```

This works but is **very inefficient** compared to real sprite batching.

### ❌ **No Sprite Sheets / Texture Atlas**

**What's missing:**

- No UV rectangle selection
- No frame animation
- No sprite sheet parsing
- No automatic batching

**Manual workaround:**

```rust
// You'd manually calculate UVs for each sprite frame
let frame_width = 1.0 / 8.0; // 8 frames wide
let frame_height = 1.0 / 4.0; // 4 frames tall

let frame_x = (current_frame % 8) as f32 * frame_width;
let frame_y = (current_frame / 8) as f32 * frame_height;

// Manually set UV coordinates (painful!)
let mut mesh = CpuMesh::square();
mesh.uvs = Some(vec![
    vec2(frame_x, frame_y),
    vec2(frame_x + frame_width, frame_y),
    vec2(frame_x + frame_width, frame_y + frame_height),
    vec2(frame_x, frame_y + frame_height),
]);

// Recreate mesh every frame (slow!)
let sprite = Mesh::new(&context, &mesh);
```

This is **extremely inefficient** - you're recreating geometry every frame.

### ❌ **No Particle System**

**What's missing:**

- No particle emitters
- No particle physics
- No particle textures
- No particle pooling

## What You Should Use Instead

Since you already have **wgpu + bevy_ecs + rapier3d**, here are better options:

### For 3D Animation: Add a Dedicated Animation Crate

**Option 1: bevy_animation (without full Bevy)**

```toml
[dependencies]
bevy_animation = { version = "0.14", default-features = false }
bevy_transform = { version = "0.14", default-features = false }
```

```rust
use bevy_ecs::prelude::*;
use bevy_animation::*;

#[derive(Component)]
struct AnimationPlayer {
    current_animation: Handle<AnimationClip>,
    time: f32,
}

// System to update animations
fn animate_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationPlayer, &mut Transform)>,
    animations: Res<Assets<AnimationClip>>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        player.time += time.delta_seconds();
        
        if let Some(clip) = animations.get(&player.current_animation) {
            // Sample animation at current time
            let sample = clip.sample(player.time);
            transform.translation = sample.translation;
            transform.rotation = sample.rotation;
        }
    }
}
```

**Features:**

- ✅ Skeletal animation
- ✅ Keyframe interpolation
- ✅ Animation blending
- ✅ GLTF animation import

**Binary cost:** ~500KB

---

**Option 2: ozz-animation-rs (Industry standard)**

```toml
[dependencies]
ozz-animation = "0.1"
```

```rust
use ozz_animation::*;

// Load skeleton and animation
let skeleton = Skeleton::from_file("skeleton.ozz")?;
let animation = Animation::from_file("walk.ozz")?;

// Sampling context
let mut context = SamplingContext::new(skeleton.num_joints());
let mut locals = vec![Transform::identity(); skeleton.num_joints()];

// Sample animation
let job = SamplingJob::new()
    .animation(&animation)
    .context(&mut context)
    .ratio(time / animation.duration())
    .output(&mut locals);

job.run();
```

**Features:**

- ✅ Production-grade (used in AAA games)
- ✅ Very efficient
- ✅ Compression support
- ✅ IK (inverse kinematics)

**Binary cost:** ~800KB

---

### For 2D Sprites: Add a Sprite Rendering Crate

**Option 1: macroquad (lightweight)**

```toml
[dependencies]
macroquad = "0.4"
```

```rust
use macroquad::prelude::*;

#[macroquad::main("Sprites")]
async fn main() {
    let texture = load_texture("spritesheet.png").await.unwrap();
    
    loop {
        clear_background(BLACK);
        
        // Draw sprite from atlas
        draw_texture_ex(
            &texture,
            x, y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    frame_x * 32.0,
                    frame_y * 32.0,
                    32.0,
                    32.0,
                )),
                ..Default::default()
            },
        );
        
        next_frame().await;
    }
}
```

**Features:**

- ✅ Sprite batching
- ✅ Texture atlas support
- ✅ Simple API
- ✅ Works with wgpu backend

**Binary cost:** ~1MB

---

**Option 2: pixels + sprite batching (manual but efficient)**

```rust
// Custom sprite batcher for wgpu
pub struct SpriteBatch {
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
    texture: wgpu::Texture,
}

impl SpriteBatch {
    pub fn draw_sprite(&mut self, rect: Rect, uv: Rect, color: Color) {
        let idx = self.vertices.len() as u32;
        
        // Add 4 vertices
        self.vertices.extend_from_slice(&[
            SpriteVertex { pos: [rect.x, rect.y], uv: [uv.x, uv.y], color },
            SpriteVertex { pos: [rect.x + rect.w, rect.y], uv: [uv.x + uv.w, uv.y], color },
            SpriteVertex { pos: [rect.x + rect.w, rect.y + rect.h], uv: [uv.x + uv.w, uv.y + uv.h], color },
            SpriteVertex { pos: [rect.x, rect.y + rect.h], uv: [uv.x, uv.y + uv.h], color },
        ]);
        
        // Add 6 indices (2 triangles)
        self.indices.extend_from_slice(&[
            idx, idx + 1, idx + 2,
            idx, idx + 2, idx + 3,
        ]);
    }
    
    pub fn flush(&mut self, encoder: &mut wgpu::CommandEncoder) {
        // Upload to GPU and draw all sprites in one call
    }
}
```

**Features:**

- ✅ Batches 1000s of sprites in one draw call
- ✅ Custom shader control
- ✅ Integrates perfectly with your wgpu setup

**Binary cost:** ~0KB (you write it)

---

**Option 3: bevy_sprite (without full Bevy)**

```toml
[dependencies]
bevy_sprite = { version = "0.14", default-features = false }
bevy_render = { version = "0.14", default-features = false }
```

```rust
use bevy_ecs::prelude::*;
use bevy_sprite::*;

#[derive(Component)]
struct Sprite {
    texture: Handle<Image>,
    rect: Option<Rect>,
    anchor: Vec2,
}

// Sprite batching happens automatically
fn render_sprites(
    query: Query<(&Sprite, &Transform)>,
    mut sprite_batch: ResMut<SpriteBatch>,
) {
    for (sprite, transform) in query.iter() {
        sprite_batch.add(sprite, transform);
    }
}
```

**Features:**

- ✅ Automatic batching
- ✅ Texture atlas support
- ✅ Integrates with bevy_ecs
- ✅ Z-ordering

**Binary cost:** ~1.5MB

---

### For Sprite Sheets: Custom Atlas Manager

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SpriteAtlas {
    frames: HashMap<String, Frame>,
}

#[derive(Deserialize)]
struct Frame {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

pub struct SpriteSheet {
    texture: wgpu::Texture,
    atlas: SpriteAtlas,
    width: u32,
    height: u32,
}

impl SpriteSheet {
    pub fn get_uv(&self, sprite_name: &str) -> Option<Rect> {
        self.atlas.frames.get(sprite_name).map(|frame| {
            Rect {
                x: frame.x as f32 / self.width as f32,
                y: frame.y as f32 / self.height as f32,
                w: frame.w as f32 / self.width as f32,
                h: frame.h as f32 / self.height as f32,
            }
        })
    }
}

// Load from TexturePacker or Aseprite JSON
let sheet = SpriteSheet::load("sprites.png", "sprites.json")?;
let uv = sheet.get_uv("player_idle_0").unwrap();
```

**Pair this with tools like:**

- TexturePacker
- Aseprite (exports JSON atlas)
- Shoebox
- Free Texture Packer

---

## Recommended Setup for Your KMP App

Given your stack (**wgpu + bevy_ecs + rapier3d + three-d**):

```toml
[dependencies]
# Existing
wgpu = "0.19"
bevy_ecs = "0.14"
rapier3d = "0.18"
three-d = "0.17"  # For 3D models/materials

# Add these:
bevy_animation = { version = "0.14", default-features = false }  # 3D animation
# Choose ONE sprite solution:

# Option A: Lightweight custom sprites (~0KB)
# - Write your own sprite batcher (100 lines)

# Option B: macroquad sprites (~1MB)
macroquad = "0.4"

# Option C: bevy_sprite (~1.5MB)
bevy_sprite = { version = "0.14", default-features = false }
```

**Architecture:**

```
Your App
├── 3D Rendering: three-d (models, materials, lighting)
├── 3D Animation: bevy_animation (skeletal)
├── 2D Sprites: Custom batcher or macroquad
├── Physics: rapier3d
├── Logic: bevy_ecs
└── GPU: wgpu (shared by all)
```

**Total binary cost:** +2-4MB depending on sprite solution

---

## Summary

| Feature | three-d | What to use instead |
|---------|---------|---------------------|
| 3D skeletal animation | ❌ Loads data only | bevy_animation or ozz-animation |
| Simple transform animation | ✅ Manual only | DIY or bevy_animation |
| 2D sprites | ❌ None | Custom batcher, macroquad, or bevy_sprite |
| Sprite sheets | ❌ None | Custom atlas + JSON parser |
| Particle systems | ❌ None | bevy_hanabi or custom |
| Frame animation | ❌ None | Custom state machine |

**Bottom line:** three-d is great for **static 3D models with PBR materials**, but you need additional crates for animation and sprites. Given you already have bevy_ecs, I'd recommend **bevy_animation** for 3D and a **custom sprite batcher** for 2D to keep binary size down.
