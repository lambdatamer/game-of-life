use bevy::prelude::*;
use bevy::render::{RenderApp, RenderStage};
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_graph::RenderGraph;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

use game_of_life::node::GameOfLifeNode;
use game_of_life::pipeline::{GameOfLifePipeline, queue_bind_group};

use crate::game_of_life;

pub struct GameOfLifeComputePlugin;

#[derive(Clone, Deref, ExtractResource)]
pub struct GameOfLifeImage(pub Handle<Image>);

#[derive(Clone, ExtractResource)]
pub struct GameOfLife {
    pub size: Size<u32>,
}

impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameOfLife {
                size: Size {
                    width: 1280,
                    height: 720,
                }
            })
            .add_plugin(ExtractResourcePlugin::<GameOfLife>::default())
            .add_plugin(ExtractResourcePlugin::<GameOfLifeImage>::default())
            .add_startup_system(setup_image);

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<GameOfLifePipeline>()
            .add_system_to_stage(RenderStage::Queue, queue_bind_group);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("game_of_life", GameOfLifeNode::default());
        render_graph.add_node_edge(
            "game_of_life",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        ).unwrap();
    }
}

fn setup_image(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    game: Res<GameOfLife>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: game.size.width,
            height: game.size.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );

    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image_handle = images.add(image);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(game.size.width as f32, game.size.height as f32)),
            ..default()
        },
        texture: image_handle.clone(),
        ..default()
    });

    commands.insert_resource(GameOfLifeImage(image_handle));
}