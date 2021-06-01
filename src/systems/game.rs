use bevy::prelude::*;
use crate::{AppState, Bot, GamePiece, Player, ThingBuilder, ThingType, Tile, Tilemap, resources::{Game, GameState, GameStateChangeEvent, Hud}};

fn initialize_game(game_pieces:&mut Query<(Entity, &GamePiece)>, commands: &mut Commands) {
    // cleanup existing entities
    game_pieces.for_each_mut(|e| {
        let mut e = commands.entity(e.0);
        e.despawn_recursive();
    });

    // create camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(GamePiece::default());

    // create tilemap
    let size = 16;
    let mut tilemap = Tilemap::new(size, 4, "tiles.png");
    for y in 0..size {
        tilemap.set_tile(Tile {
            index:1,
            solid:true,
            ..Default::default()
        }, 0, y);
        tilemap.set_tile(Tile {
            index:1,
            solid:true,
            ..Default::default()
        }, size-1, y);
    }

    for x in 0..size {
        tilemap.set_tile(Tile {
            index:1,
            solid:true,
            ..Default::default()
        }, x, 0);
        tilemap.set_tile(Tile {
            index:1,
            solid:true,
            ..Default::default()
        }, x, size - 1);
    }

    for y in 0..size {
        for x in 0..size {
            if x % 5 == 0 {
                if y % 5 == 0 {
                    tilemap.set_tile(Tile {
                        index:1,
                        solid:true,
                        ..Default::default()
                    }, x, y);
                }
            }
        }
    }

    commands.spawn().insert(tilemap);

    // spawn player
    commands.spawn().insert(ThingBuilder {
        translation:Vec3::new(2.5, 2.5, 0.0),
        rotation:Quat::default(),
        thing_type:ThingType::Tank,
        ..Default::default()
    })
    .insert(Player::default());

    let mut spawn_bot = |x, y| {
        commands.spawn().insert(ThingBuilder {
            translation:Vec3::new(x, y, 0.0),
            rotation:Quat::default(),
            thing_type:ThingType::Tank,
            ..Default::default()
        })
        .insert(Bot::default());
    };

    // spawn bot
    spawn_bot(size as f32 - 2.5, size as f32 - 2.5);
    spawn_bot(2.5, size as f32 - 2.5);
    spawn_bot(size as f32 - 2.5, 2.5);
}

pub fn game_system(mut game:ResMut<Game>, mut commands: Commands, mut game_pieces:Query<(Entity, &GamePiece)>, mut game_state_change_reader:EventReader<GameStateChangeEvent>, mut hud:ResMut<Hud>, time:Res<Time>, players:Query<&Player>, mut app_state:ResMut<State<AppState>>) {
    for e in game_state_change_reader.iter() {
        match e.to {
            GameState::NotSet => {
            }
            GameState::GetReady => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "Get Ready!".into();
                initialize_game(&mut game_pieces, &mut commands);
                game.transition(GameState::Go, 3.0, &time);
            }
            GameState::Go => {
                let _ = app_state.set(AppState::InGame);
                hud.center_text = "Go!".into();
                game.transition(GameState::InProgress, 1.0, &time);
            }
            GameState::InProgress => {
                let _ = app_state.set(AppState::InGame);
                hud.center_text = "".into();
            }
            GameState::Loading => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "Loading...".into();
                game.transition_asap(GameState::GetReady);
            }
            GameState::Restarting => {
                let _ = app_state.set(AppState::InBetweenGames);
                hud.center_text = "Failure!\nRestarting level...".into();
                game.transition(GameState::GetReady, 3.0, &time);
            }
        }

    }

    if game.state == GameState::InProgress {

        let mut player_is_dead = true;
        players.for_each(|_| {
            player_is_dead = false;
        });

    
        if player_is_dead {
            if game.next_state() == None {

                game.transition(GameState::Restarting, 3.0, &time);
            }
        }
    }
   
}

pub fn game_tick_system(mut game:ResMut<Game>, time:Res<Time>, mut game_state_change_writer:EventWriter<GameStateChangeEvent>) {
    if let Some((next_state, at)) = game.next_state_at {
        if at <= time.seconds_since_startup() {
            let prev = game.state;
            game.state = next_state;
            game.clear_transition();
            game_state_change_writer.send(GameStateChangeEvent {
                from:prev,
                to:game.state
            });
        }
    }
}
 
