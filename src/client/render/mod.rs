//! Small wgpu sprite path kept downstream of renderer-neutral presentation.

use std::fmt;
use std::mem;
use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use winit::dpi::PhysicalSize;
use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use super::assets::build_prototype_atlas;
use super::model::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use super::presentation::{ScenePresentation, SpriteDraw, logical_viewport};

const MAX_VERTICES: usize = 8_192;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    tint: [f32; 4],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

    const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct SpriteRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    pipeline: wgpu::RenderPipeline,
    atlas_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
}

impl SpriteRenderer {
    pub async fn new(
        window: Arc<Window>,
        display_handle: OwnedDisplayHandle,
    ) -> Result<Self, RenderError> {
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            return Err(RenderError::ZeroSizedSurface);
        }

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor::new_with_display_handle_from_env(Box::new(display_handle)),
        );
        let surface = instance
            .create_surface(window)
            .map_err(|error| RenderError::SurfaceCreation(error.to_string()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .map_err(|error| RenderError::AdapterUnavailable(error.to_string()))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Hollow Grove client device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .map_err(|error| RenderError::DeviceRequest(error.to_string()))?;

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or(RenderError::UnsupportedSurface)?;
        config.present_mode = wgpu::PresentMode::Fifo;
        surface.configure(&device, &config);

        let atlas = build_prototype_atlas();
        let expected_bytes = (atlas.width * atlas.height * 4) as usize;
        if atlas.rgba8.len() != expected_bytes {
            return Err(RenderError::InvalidTextureData {
                expected: expected_bytes,
                actual: atlas.rgba8.len(),
            });
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Hollow Grove prototype sprite atlas"),
            size: wgpu::Extent3d {
                width: atlas.width,
                height: atlas.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &atlas.rgba8,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(atlas.width * 4),
                rows_per_image: Some(atlas.height),
            },
            wgpu::Extent3d {
                width: atlas.width,
                height: atlas.height,
                depth_or_array_layers: 1,
            },
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Hollow Grove nearest-neighbor sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let atlas_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Hollow Grove sprite atlas layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
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
        });
        let atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Hollow Grove sprite atlas bind group"),
            layout: &atlas_layout,
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
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Hollow Grove sprite shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sprite.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Hollow Grove sprite pipeline layout"),
            bind_group_layouts: &[Some(&atlas_layout)],
            immediate_size: 0,
        });
        let vertex_buffers = [Some(Vertex::layout())];
        let color_targets = [Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Hollow Grove sprite pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &vertex_buffers,
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &color_targets,
            }),
            multiview_mask: None,
            cache: None,
        });
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Hollow Grove dynamic sprite vertices"),
            size: (MAX_VERTICES * mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            atlas_bind_group,
            vertex_buffer,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, scene: &ScenePresentation) -> Result<(), RenderError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }
        let vertices = scene_vertices(scene)?;
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Timeout => {
                return Err(RenderError::SurfaceFrame(SurfaceFrameIssue::Timeout));
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(RenderError::SurfaceFrame(SurfaceFrameIssue::Occluded));
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                return Err(RenderError::SurfaceFrame(SurfaceFrameIssue::Outdated));
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err(RenderError::SurfaceFrame(SurfaceFrameIssue::Lost));
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RenderError::SurfaceFrame(SurfaceFrameIssue::Validation));
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Hollow Grove frame encoder"),
            });
        let color_attachments = [Some(wgpu::RenderPassColorAttachment {
            view: &view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.035,
                    g: 0.065,
                    b: 0.095,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })];
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Hollow Grove sprite pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            let viewport = logical_viewport(self.size.width, self.size.height);
            pass.set_viewport(
                viewport.x,
                viewport.y,
                viewport.width,
                viewport.height,
                0.0,
                1.0,
            );
            pass.set_scissor_rect(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width as u32,
                viewport.height as u32,
            );
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.atlas_bind_group, &[]);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..vertices.len() as u32, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);
        Ok(())
    }
}

fn scene_vertices(scene: &ScenePresentation) -> Result<Vec<Vertex>, RenderError> {
    let vertex_count = scene.sprites.len().saturating_mul(6);
    if vertex_count > MAX_VERTICES {
        return Err(RenderError::TooManyVertices {
            requested: vertex_count,
            maximum: MAX_VERTICES,
        });
    }
    let mut vertices = Vec::with_capacity(vertex_count);
    for sprite in &scene.sprites {
        push_sprite_vertices(&mut vertices, sprite);
    }
    Ok(vertices)
}

fn push_sprite_vertices(vertices: &mut Vec<Vertex>, sprite: &SpriteDraw) {
    let left = logical_x(sprite.screen_x as f32);
    let right = logical_x(sprite.screen_x as f32 + sprite.width as f32);
    let top = logical_y(sprite.screen_y as f32);
    let bottom = logical_y(sprite.screen_y as f32 + sprite.height as f32);
    let [u0, v0, u1, v1] = sprite.source.normalized();
    let tint = [1.0; 4];
    vertices.extend([
        Vertex {
            position: [left, top],
            uv: [u0, v0],
            tint,
        },
        Vertex {
            position: [left, bottom],
            uv: [u0, v1],
            tint,
        },
        Vertex {
            position: [right, bottom],
            uv: [u1, v1],
            tint,
        },
        Vertex {
            position: [left, top],
            uv: [u0, v0],
            tint,
        },
        Vertex {
            position: [right, bottom],
            uv: [u1, v1],
            tint,
        },
        Vertex {
            position: [right, top],
            uv: [u1, v0],
            tint,
        },
    ]);
}

fn logical_x(value: f32) -> f32 {
    value / LOGICAL_WIDTH as f32 * 2.0 - 1.0
}

fn logical_y(value: f32) -> f32 {
    1.0 - value / LOGICAL_HEIGHT as f32 * 2.0
}

#[derive(Debug)]
pub enum RenderError {
    ZeroSizedSurface,
    SurfaceCreation(String),
    AdapterUnavailable(String),
    DeviceRequest(String),
    UnsupportedSurface,
    InvalidTextureData { expected: usize, actual: usize },
    TooManyVertices { requested: usize, maximum: usize },
    SurfaceFrame(SurfaceFrameIssue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceFrameIssue {
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}

impl RenderError {
    #[must_use]
    pub const fn surface_is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::SurfaceFrame(SurfaceFrameIssue::Lost | SurfaceFrameIssue::Outdated)
        )
    }

    #[must_use]
    pub const fn surface_is_timeout(&self) -> bool {
        matches!(
            self,
            Self::SurfaceFrame(SurfaceFrameIssue::Timeout | SurfaceFrameIssue::Occluded)
        )
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSizedSurface => write!(formatter, "window surface has zero size"),
            Self::SurfaceCreation(error) => write!(formatter, "surface creation failed: {error}"),
            Self::AdapterUnavailable(error) => {
                write!(formatter, "GPU adapter unavailable: {error}")
            }
            Self::DeviceRequest(error) => write!(formatter, "GPU device request failed: {error}"),
            Self::UnsupportedSurface => {
                write!(formatter, "adapter has no supported surface configuration")
            }
            Self::InvalidTextureData { expected, actual } => write!(
                formatter,
                "prototype texture contains {actual} bytes; expected {expected}"
            ),
            Self::TooManyVertices { requested, maximum } => write!(
                formatter,
                "sprite scene requested {requested} vertices; maximum is {maximum}"
            ),
            Self::SurfaceFrame(error) => write!(formatter, "surface frame failed: {error}"),
        }
    }
}

impl std::error::Error for RenderError {}

impl fmt::Display for SurfaceFrameIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Timeout => "timed out",
            Self::Occluded => "is occluded",
            Self::Outdated => "is outdated",
            Self::Lost => "was lost",
            Self::Validation => "failed validation",
        };
        write!(formatter, "surface frame {label}")
    }
}
