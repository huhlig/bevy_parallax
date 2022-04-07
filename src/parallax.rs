use bevy::{
    ecs::{
        bundle::Bundle,
        system::{lifetimeless::SRes, SystemParamItem},
    },
    prelude::{
        App, AssetServer, ComputedVisibility, GlobalTransform, Handle, Image, Plugin,
        Shader, Transform, Visibility,
    },
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, SamplerBindingType,
            SamplerDescriptor, ShaderStages, TextureSampleType, TextureViewDimension,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, Mesh2dHandle},
};

/// Plugin to Enable Parallax Backgrounds
#[derive(Default)]
pub struct ParallaxBackgroundPlugin;

impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<ParallaxBackgroundMaterial>::default());
    }
}

/// Each Bundle represents a single layer of the parallax Background
#[derive(Bundle, Clone)]
pub struct ParallaxBackgroundBundle {
    /// Must consist of 6 vertices. Should be a child of the camera
    pub mesh: Mesh2dHandle,
    /// Handle to the Material
    pub material: Handle<ParallaxBackgroundMaterial>,
    /// Local Transform, should be (0.0, 0.0, Z-Layer)
    pub transform: Transform,
    /// Global Transform calculated by renderer
    pub global_transform: GlobalTransform,
    /// Visibility of the Background
    pub visibility: Visibility,
    /// Computed Visibility from parent objects
    pub computed_visibility: ComputedVisibility,
}

impl Default for ParallaxBackgroundBundle {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

/// Control the speed of the X and Y Parallax
#[derive(AsStd140, Clone, Debug)]
pub struct ParallaxParameters {
    /// Speed of Parallax on the X axis
    pub x_speed: f32,
    /// Speed of Parallax on the Y axis
    pub y_speed: f32,
}

impl Default for ParallaxParameters {
    fn default() -> Self {
        Self {
            x_speed: 0.0,
            y_speed: 0.0,
        }
    }
}

/// Parallax Material containing Image Handle and Parameters
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "db5b7729-2aad-4e0d-93ed-afb86077173a"]
pub struct ParallaxBackgroundMaterial {
    pub parameters: ParallaxParameters,
    pub texture: Handle<Image>,
}

impl Default for ParallaxBackgroundMaterial {
    fn default() -> Self {
        ParallaxBackgroundMaterial {
            parameters: Default::default(),
            texture: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct GpuParallaxBackgroundMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for ParallaxBackgroundMaterial {
    type ExtractedAsset = Self;
    type PreparedAsset = GpuParallaxBackgroundMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<RenderAssets<Image>>,
        SRes<Material2dPipeline<Self>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, gpu_images, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let gpu_image = match gpu_images.get(&extracted_asset.texture) {
            Some(gpu_image) => gpu_image,
            // if the image isn't loaded yet, try next frame
            None => return Err(PrepareAssetError::RetryNextUpdate(extracted_asset)),
        };

        let sampler = render_device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            ..Default::default()
        });

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: extracted_asset.parameters.as_std140().as_bytes(),
            label: Some("Parallax Background Parameters Buffer Descriptor"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_image.texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer.as_entire_binding(),
                },
            ],
            label: Some("Parallax Background Bind Group Descriptor"),
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuParallaxBackgroundMaterial { bind_group })
    }
}

impl Material2d for ParallaxBackgroundMaterial {
    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            ParallaxParameters::std140_size_static() as u64
                        ),
                    },
                    count: None,
                },
            ],
            label: None,
        })
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/parallax.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/parallax.wgsl"))
    }
}
