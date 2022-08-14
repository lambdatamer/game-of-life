use bevy::prelude::*;
use bevy::render::render_graph;
use bevy::render::render_graph::{NodeRunError, RenderGraphContext};
use bevy::render::render_resource::{CachedPipelineState, ComputePassDescriptor, PipelineCache};
use bevy::render::renderer::RenderContext;

use crate::game_of_life::pipeline::{GameOfLifeImageBindGroup, GameOfLifePipeline};
use crate::game_of_life::plugin::GameOfLife;

const WORKGROUP_SIZE: u32 = 8;

enum GameOfLifeState {
    Loading,
    Init,
    Update,
}

pub struct GameOfLifeNode {
    state: GameOfLifeState,
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}

impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GameOfLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            GameOfLifeState::Loading => {
                if let CachedPipelineState::Ok(_) = pipeline_cache
                    .get_compute_pipeline_state(pipeline.init_pipeline) {
                    self.state = GameOfLifeState::Init;
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) = pipeline_cache
                    .get_compute_pipeline_state(pipeline.update_pipeline) {
                    self.state = GameOfLifeState::Update;
                }
            }
            GameOfLifeState::Update => {}
        }
    }

    fn run(&self,
           _graph: &mut RenderGraphContext,
           render_context: &mut RenderContext,
           world: &World,
    ) -> Result<(), NodeRunError> {
        let game = world.resource::<GameOfLife>();
        let texture_bind_group = &world.resource::<GameOfLifeImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<GameOfLifePipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    game.size.width / WORKGROUP_SIZE,
                    game.size.height / WORKGROUP_SIZE,
                    1,
                )
            }
            GameOfLifeState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    game.size.width / WORKGROUP_SIZE,
                    game.size.height / WORKGROUP_SIZE,
                    1,
                )
            }
        }

        Ok(())
    }
}