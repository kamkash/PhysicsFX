use nalgebra as na;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: na::Point3<f32>,
    pub target: na::Point3<f32>,
    pub up: na::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32, // in degrees
    pub znear: f32,
    pub zfar: f32,
    pub is_orthographic: bool,
    pub ortho_size: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> na::Matrix4<f32> {
        let view = na::Matrix4::look_at_rh(&self.eye, &self.target, &self.up);

        let proj = if self.is_orthographic {
            let half_size = self.ortho_size / 2.0;
            let left = -half_size * self.aspect;
            let right = half_size * self.aspect;
            let bottom = -half_size;
            let top = half_size;
            na::Orthographic3::new(left, right, bottom, top, self.znear, self.zfar).to_homogeneous()
        } else {
            na::Perspective3::new(self.aspect, self.fovy.to_radians(), self.znear, self.zfar)
                .to_homogeneous()
        };

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: na::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
