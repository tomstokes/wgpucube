use glam::{Mat4, Vec3};
use wgpu::TextureFormat;
use wgpu::util::DeviceExt;

const CUBE_VERTICES: [[f32; 3]; 24] = [
    // Front
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    // Back
    [1.0, -1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    // Right
    [1.0, -1.0, 1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, -1.0],
    // Left
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, 1.0, 1.0],
    // Top
    [-1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [1.0, 1.0, -1.0],
    // Bottom
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
];

const CUBE_COLORS: [[f32; 3]; 24] = [
    // Front
    [0.0, 0.0, 1.0], // blue
    [1.0, 0.0, 1.0], // magenta
    [0.0, 1.0, 1.0], // cyan
    [1.0, 1.0, 1.0], // white
    // Back
    [1.0, 0.0, 0.0], // red
    [0.0, 0.0, 0.0], // black
    [1.0, 1.0, 0.0], // yellow
    [0.0, 1.0, 0.0], // green
    // Right
    [1.0, 0.0, 1.0], // magenta
    [1.0, 0.0, 0.0], // red
    [1.0, 1.0, 1.0], // white
    [1.0, 1.0, 0.0], // yellow
    // Left
    [0.0, 0.0, 0.0], // black
    [0.0, 0.0, 1.0], // blue
    [0.0, 1.0, 0.0], // green
    [0.0, 1.0, 1.0], // cyan
    // Top
    [0.0, 1.0, 1.0], // cyan
    [1.0, 1.0, 1.0], // white
    [0.0, 1.0, 0.0], // green
    [1.0, 1.0, 0.0], // yellow
    // Bottom
    [0.0, 0.0, 0.0], // black
    [1.0, 0.0, 0.0], // red
    [0.0, 0.0, 1.0], // blue
    [1.0, 0.0, 1.0], // magenta
];

const CUBE_NORMALS: [[f32; 3]; 24] = [
    // Front
    [0.0, 0.0, 1.0], // forward
    [0.0, 0.0, 1.0], // forward
    [0.0, 0.0, 1.0], // forward
    [0.0, 0.0, 1.0], // forward
    // Back
    [0.0, 0.0, -1.0], // backward
    [0.0, 0.0, -1.0], // backward
    [0.0, 0.0, -1.0], // backward
    [0.0, 0.0, -1.0], // backward
    // Right
    [1.0, 0.0, 0.0], // right
    [1.0, 0.0, 0.0], // right
    [1.0, 0.0, 0.0], // right
    [1.0, 0.0, 0.0], // right
    // Left
    [-1.0, 0.0, 0.0], // left
    [-1.0, 0.0, 0.0], // left
    [-1.0, 0.0, 0.0], // left
    [-1.0, 0.0, 0.0], // left
    // Top
    [0.0, 1.0, 0.0], // up
    [0.0, 1.0, 0.0], // up
    [0.0, 1.0, 0.0], // up
    [0.0, 1.0, 0.0], // up
    // Bottom
    [0.0, -1.0, 0.0], // down
    [0.0, -1.0, 0.0], // down
    [0.0, -1.0, 0.0], // down
    [0.0, -1.0, 0.0], // down
];

const CUBE_INDICES: [u16; 36] = [
    // Each face is composed of 2 triangles, therefore there are 6 indices per face
    0, 1, 2, 1, 3, 2, // Front
    4, 5, 6, 5, 7, 6, // Back
    8, 9, 10, 9, 11, 10, // Right
    12, 13, 14, 13, 15, 14, // Left
    16, 17, 18, 17, 19, 18, // Top
    20, 21, 22, 21, 23, 22, // Bottom
];

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x3,
    ];

    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    model_view: [[f32; 4]; 4],
    model_view_projection: [[f32; 4]; 4],
    // The normal matrix is a 3x3 matrix, but WebGPU requires alignment 16 for matrices.
    // This is passed as a 4x4 matrix for simplicity. Technically, the last column could be dropped
    // to save 16 bytes, but it's simpler to pass a 4x4 matrix.
    normal: [[f32; 4]; 4],
}

fn create_vertices<const N: usize>(
    vertices: &[[f32; 3]; N],
    colors: &[[f32; 3]; N],
    normals: &[[f32; 3]; N],
) -> Vec<Vertex> {
    (0..N)
        .map(|i| Vertex {
            position: vertices[i],
            color: colors[i],
            normal: normals[i],
        })
        .collect()
}

#[derive(Debug)]
pub(crate) struct Cube {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    aspect_ratio: f32,
    step: u64,
}

impl Cube {
    pub fn new(texture_format: TextureFormat, device: &wgpu::Device) -> Self {
        // Create vertex and index buffers
        let vertices = create_vertices(&CUBE_VERTICES, &CUBE_COLORS, &CUBE_NORMALS);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(&CUBE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create uniform buffer
        let step = 0;
        let aspect_ratio = 1.0_f32; // TODO: Use correct aspect ratio (width/height)
        let uniforms = Self::uniforms(step, aspect_ratio);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create render pipeline
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cube Uniform Buffer Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Uniforms>() as u64),
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Uniform Buffer BindGroup"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("cube_smooth.wgsl"));
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::buffer_layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None, // Depth stencil is omitted for this simple example
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(texture_format.into())],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            pipeline,
            bind_group,
            aspect_ratio,
            step,
        }
    }

    fn uniforms(step: u64, aspect_ratio: f32) -> Uniforms {
        // Calculate transformation matrices
        //
        // This attemps to replicate the behavior of kmscube's custom transformation code which
        // keeps the vertical FOV fixed.
        let model_view = Mat4::from_translation(Vec3::new(0.0, 0.0, -8.0))
            * Mat4::from_rotation_x((45.0 + 0.25 * step as f32).to_radians())
            * Mat4::from_rotation_y((45.0 - 0.5 * step as f32).to_radians())
            * Mat4::from_rotation_z((10.0 + 0.15 * step as f32).to_radians());
        let top = 2.8 * (1.0 / aspect_ratio);
        let near = 6.0;
        let far = 10.0;
        let fov_y = 2.0 * (top / near).atan(); // Equivalent vertical FOV
        let projection = Mat4::perspective_rh(fov_y, aspect_ratio, near, far);
        let model_view_projection = projection * model_view;
        let normal = model_view.inverse().transpose();

        // Create uniform buffer
        Uniforms {
            model_view: model_view.to_cols_array_2d(),
            model_view_projection: model_view_projection.to_cols_array_2d(),
            normal: normal.to_cols_array_2d(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, queue: &wgpu::Queue) {
        self.aspect_ratio = new_size.width as f32 / new_size.height as f32;
        let uniforms = Self::uniforms(self.step, self.aspect_ratio);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    pub fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        // Update uniform buffer to animate the cube
        let uniforms = Self::uniforms(self.step, self.aspect_ratio);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        self.step += 1;

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Cube Encoder"),
        });
        {
            // The render pass returned by `begin_render_pass` has a lifetime relationship with the
            // `encoder` to ensure that the `CommandEncoder` cannot be mutated while the
            // `RenderPass` is being used to record commands.
            //
            // As a result, the `render_pass` must be dropped before the `encoder` can be submitted
            // to the queue. The `render_pass` is placed in a nested scope in this example to allow
            // it to be dropped before submission to the queue. Alternatively, an explicit
            // `drop(render_pass)` call could be used for the same result.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cube Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..36, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }
}
