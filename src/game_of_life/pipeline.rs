use std::borrow::Cow;

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, ComputePipelineDescriptor, PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension};
use bevy::render::render_resource::{BindGroupLayout, CachedComputePipelineId};
use bevy::render::renderer::RenderDevice;

use crate::game_of_life::plugin::GameOfLifeImage;

const SHADER: &str = "shaders\\game_of_life.wgsl";

pub struct GameOfLifePipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for GameOfLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("game_of_life_bind_group_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });

        let shader = world
            .resource::<AssetServer>()
            .load(SHADER);

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        GameOfLifePipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

pub struct GameOfLifeImageBindGroup(pub BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<GameOfLifePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    game_of_life_image: Res<GameOfLifeImage>,
    render_device: Res<RenderDevice>,
) {
    let view = &gpu_images[&game_of_life_image.0];

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("game_of_life_bind_group"),
        layout: &(pipeline.texture_bind_group_layout),
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&view.texture_view),
        }],
    });

    commands.insert_resource(GameOfLifeImageBindGroup(bind_group))
}