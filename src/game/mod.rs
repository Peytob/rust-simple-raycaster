mod graphics;
mod game_state;
mod ecs;
mod event;
mod model;

use std::time::Duration;
use ecs_rust::world::World;
use glm::vec2;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use crate::game::ecs::component::direction_component::DirectionComponent;
use crate::game::ecs::component::player_flag_component::PlayerFlagComponent;
use crate::game::ecs::component::position_component::PositionComponent;
use crate::game::ecs::component::tilemap_component::TilemapComponent;
use crate::game::ecs::system::moving_system::MovingSystem;
use crate::game::event::events::Events;
use crate::game::game_state::{GameState, Repositories};
use crate::game::graphics::Graphics;
use crate::game::model::tile::Tile;
use crate::game::model::tilemap::Tilemap;

pub struct Game {
    graphics: Graphics,
    events: Events,
    world: World,

    game_state: GameState
}

impl Game {

    pub fn initialize_game(sdl_context: Sdl) -> Game {
        log::info!("Initializing game");

        let mut graphics = Graphics::initialize_graphics(&sdl_context);
        let event_pump = sdl_context.event_pump().unwrap();

        log::info!("Game has been initialized");

        let mut events = Events::new(event_pump);

        log::info!("Loading resources");

        let game_state = GameState::new();

        Game::load_resources(game_state.repositories());

        log::info!("Resources loaded");

        log::info!("Initializing ECS world");

        let world = Game::create_ecs_world(&mut graphics, &mut events);

        log::info!("ECS world has been initialized");

            Game {
                graphics,
                events,
                world,

                game_state
            }
        }

        fn create_ecs_world(graphics: &mut Graphics, events: &mut Events) -> World {
            let mut world = World::new();

            // Registering components
            world
                .register_component::<PositionComponent>()
                .register_component::<DirectionComponent>()
                .register_component::<TilemapComponent>()
                .register_component::<PlayerFlagComponent>();

            // Creating systems
            world
                // Input and events handling systems
                .add_system(MovingSystem::new(&events.event_pump()));

            // Creating entities
            {
                let player_entity_id = world.create_entity();

                log::info!("Creating player entity with id {}", player_entity_id);

                world.add_component_to_entity(player_entity_id, PositionComponent::new(vec2(3.0, 3.0)));
                world.add_component_to_entity(player_entity_id, DirectionComponent::new(0.0f32));
                world.add_component_to_entity(player_entity_id, PlayerFlagComponent::new());
            }

            {
                let tilemap_entity_id = world.create_entity();
                world.add_component_to_entity(tilemap_entity_id, TilemapComponent::new(1));
            }

            world
        }

    fn load_resources(repositories: &Repositories) {
        // TODO Load resources from file

        repositories.tiles_repository().borrow_mut()
            .register_resource(Tile::new(0))
            .register_resource(Tile::new(1))
            .register_resource(Tile::new(2))
            .register_resource(Tile::new(3));

        let tiles = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 2, 2, 2, 0, 0, 1],
            vec![1, 0, 0, 0, 2, 0, 2, 0, 0, 1],
            vec![1, 0, 0, 0, 2, 2, 2, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        ];

        repositories.tilemap_repository().borrow_mut()
            .register_resource(Tilemap::from_raw_tilemap(1, tiles));
    }

        pub fn run_game_loop(&mut self) {

            'main_game_loop: loop {
                self.handle_events();

                self.world.update();

                if !self.game_state.is_game_running() {
                    break 'main_game_loop;
                }

                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            }
        }

        fn handle_events(&mut self) {
            for event in self.events.event_pump().borrow_mut().poll_iter() {
                use sdl2::event::Event;

                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.game_state.set_is_game_running(false)
                },

                _ => {}
            }
        }
    }
}
