use bevy::prelude::*;
use bevy::render::render_graph;
use bevy::render::render_graph::{NodeRunError, RenderGraphContext};
use bevy::render::render_resource::{CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor, PipelineCache};
use bevy::render::renderer::RenderContext;

use crate::game_of_life::pipeline::{GameOfLifeImageBindGroup, GameOfLifePipeline};
use crate::game_of_life::plugin::GameOfLife;

const WORKGROUP_SIZE: u32 = 8;

pub struct GameOfLifeNode {
    state: GameOfLifeState
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading
        }
    }
}

enum GameOfLifeState {
    Loading,
    Init,
    Running,
}


impl GameOfLifeNode {
    fn begin_pass(
        &self,
        world: &World,
        render_context: &mut RenderContext,
        pipeline_id: CachedComputePipelineId,
    ) {
        let game = world.resource::<GameOfLife>();
        let texture_bind_group = &world.resource::<GameOfLifeImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        let pipeline = pipeline_cache
            .get_compute_pipeline(pipeline_id)
            .unwrap();

        pass.set_pipeline(pipeline);
        pass.dispatch_workgroups(
            game.size.width / WORKGROUP_SIZE,
            game.size.height / WORKGROUP_SIZE,
            1,
        )
    }
}

impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GameOfLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            GameOfLifeState::Loading => {
                if let CachedPipelineState::Ok(_) = pipeline_cache
                    .get_compute_pipeline_state(pipeline.init_pipeline) {};
                self.state = GameOfLifeState::Init
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) = pipeline_cache
                    .get_compute_pipeline_state(pipeline.update_pipeline) {};
                self.state = GameOfLifeState::Running
            }
            GameOfLifeState::Running => {}
        };
    }

    fn run(&self,
           _graph: &mut RenderGraphContext,
           render_context: &mut RenderContext,
           world: &World,
    ) -> Result<(), NodeRunError> {
        let game = world.resource::<GameOfLife>();
        let pipeline = world.resource::<GameOfLifePipeline>();

        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                self.begin_pass(world, render_context, pipeline.init_pipeline);
            }
            GameOfLifeState::Running => {
                if !game.is_paused {
                    self.begin_pass(world, render_context, pipeline.update_pipeline);
                }
            }
        }

        Ok(())
    }
}
