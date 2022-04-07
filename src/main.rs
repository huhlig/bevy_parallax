use bevy::{DefaultPlugins, prelude::*};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::parallax::{ParallaxBackgroundBundle, ParallaxBackgroundMaterial, ParallaxParameters};

mod parallax;

const SPEED_C: f32 = 5.0;
const SPEED_F1: f32 = 16.0;
const SPEED_F2: f32 = 16.0;
const SPEED_F3: f32 = 32.0;
const SPEED_F4: f32 = 32.0;
const SPEED_B2: f32 = 0.001;
const SPEED_B1: f32 = 0.01;
const SPEED_B0: f32 = 0.1;
const LAYER_B2: f32 = 000.0;
const LAYER_B1: f32 = 010.0;
const LAYER_B0: f32 = 100.0;
const LAYER_F1: f32 = 200.0;
const LAYER_F2: f32 = 300.0;
const LAYER_F3: f32 = 400.0;
const LAYER_F4: f32 = 500.0;

pub struct LayerEntities {
    b2: Entity,
    b1: Entity,
    b0: Entity,
    f1: Entity,
    f2: Entity,
    f3: Entity,
    f4: Entity,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GameState {
    Example
}

#[derive(Component)]
pub struct Movement {
    d: bool,
    x: f32,
    y: f32,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Component)]
pub struct Dolly {
    direction: Direction,
}

fn main() {
    let mut builder = App::new();
    builder
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(parallax::ParallaxBackgroundPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_system_set(SystemSet::on_enter(GameState::Example).with_system(on_enter))
        .add_system_set(
            SystemSet::on_update(GameState::Example)
                .with_system(update_objects)
                .with_system(update_camera)
                .with_system(handle_input),
        )
        .add_system_set(SystemSet::on_exit(GameState::Example).with_system(on_exit))
        .add_state(GameState::Example)
        .run();
}

fn on_enter(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParallaxBackgroundMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    let background_mesh: Mesh2dHandle = meshes
        .add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))).into())
        .into();
    let bg0 = commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: background_mesh.clone(),
        transform: Transform::from_xyz(0.0, 0.0, LAYER_B0),
        material: materials
            .add(
                ParallaxBackgroundMaterial {
                    parameters: ParallaxParameters {
                        x_speed: SPEED_B0,
                        y_speed: SPEED_B0,
                    },
                    texture: asset_server.load("backgrounds/bg0.png").into(),
                },
            )
            .into(),
        ..Default::default()
    }).id();
    let bg1 = commands.spawn_bundle(ParallaxBackgroundBundle {
        mesh: background_mesh.clone(),
        transform: Transform::from_xyz(0.0, 0.0, LAYER_B1),
        material: materials
            .add(
                ParallaxBackgroundMaterial {
                    parameters: ParallaxParameters {
                        x_speed: SPEED_B1,
                        y_speed: SPEED_B1,
                    },
                    texture: asset_server.load("backgrounds/bg1.png").into(),
                },
            )
            .into(),
        ..Default::default()
    }).id();
    let bg2 = commands.spawn_bundle(ParallaxBackgroundBundle {
        mesh: background_mesh.clone(),
        transform: Transform::from_xyz(0.0, 0.0, LAYER_B2),
        material: materials
            .add(
                ParallaxBackgroundMaterial {
                    parameters: ParallaxParameters {
                        x_speed: SPEED_B2,
                        y_speed: SPEED_B2,
                    },
                    texture: asset_server.load("backgrounds/bg2.png").into(),
                },
            )
            .into(),
        ..Default::default()
    }).id();
    let mut entities = None;
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Dolly { direction: Direction::North }).with_children(|parent| {
        let fg1 = parent.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -999.0 + LAYER_F1),
            texture: asset_server.load("sprites/baren.png").into(),
            ..Default::default()
        }).insert(Movement {
            d: true,
            x: SPEED_F1,
            y: 0.0,
        }).id();
        let fg2 = parent.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -999.0 + LAYER_F2),
            texture: asset_server.load("sprites/ice.png").into(),
            ..Default::default()
        }).insert(Movement {
            d: true,
            x: 0.0,
            y: SPEED_F2,
        }).id();
        let fg3 = parent.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -999.0 + LAYER_F3),
            texture: asset_server.load("sprites/lava.png").into(),
            ..Default::default()
        }).insert(Movement {
            d: true,
            x: SPEED_F3,
            y: SPEED_F3,
        }).id();
        let fg4 = parent.spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -999.0 + LAYER_F4),
            texture: asset_server.load("sprites/terran.png").into(),
            ..Default::default()
        }).insert(Movement {
            d: true,
            x: -SPEED_F4,
            y: SPEED_F4,
        }).id();
        entities = Some(LayerEntities {
            b2: bg2,
            b1: bg1,
            b0: bg0,
            f1: fg1,
            f2: fg2,
            f3: fg3,
            f4: fg4,
        });
    });
    commands.insert_resource(entities.unwrap());
}

fn update_objects(time: Res<Time>, mut query: Query<(&mut Movement, &mut Transform)>) {
    for (mut movement, mut transform) in query.iter_mut() {
        if movement.d {
            transform.translation.x += movement.x * time.delta_seconds_f64() as f32;
            transform.translation.y += movement.y * time.delta_seconds_f64() as f32;
        } else {
            transform.translation.x -= movement.x * time.delta_seconds_f64() as f32;
            transform.translation.y -= movement.y * time.delta_seconds_f64() as f32;
        }
        if transform.translation.x.abs() > 200.0 || transform.translation.y.abs() > 200.0 {
            movement.d = !movement.d;
        }
    }
}

fn update_camera(time: Res<Time>, mut query: Query<(&mut Transform, &mut Dolly)>) {
    for (mut transform, mut dolly) in query.iter_mut() {
        match dolly.direction {
            Direction::North => {
                transform.translation.y += SPEED_C * time.delta_seconds_f64() as f32;
                if transform.translation.y > 500.0 {
                    dolly.direction = Direction::East
                }
            }
            Direction::East => {
                transform.translation.x += SPEED_C * time.delta_seconds_f64() as f32;
                if transform.translation.x > 500.0 {
                    dolly.direction = Direction::South
                }
            }
            Direction::South => {
                transform.translation.y -= SPEED_C * time.delta_seconds_f64() as f32;
                if transform.translation.y < -500.0 {
                    dolly.direction = Direction::West
                }
            }
            Direction::West => {
                transform.translation.x -= SPEED_C * time.delta_seconds_f64() as f32;
                if transform.translation.x < -1000.0 {
                    dolly.direction = Direction::North
                }
            }
        }
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    entities: Res<LayerEntities>,
    mut query: Query<&mut Visibility>,
) {
    if keys.just_pressed(KeyCode::Key1) {
        if let Ok(mut visibility) = query.get_mut(entities.f1) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::Key2) {
        if let Ok(mut visibility) = query.get_mut(entities.f2) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::Key3) {
        if let Ok(mut visibility) = query.get_mut(entities.f3) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::Key4) {
        if let Ok(mut visibility) = query.get_mut(entities.f4) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::Q) {
        if let Ok(mut visibility) = query.get_mut(entities.b0) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::W) {
        if let Ok(mut visibility) = query.get_mut(entities.b1) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::E) {
        if let Ok(mut visibility) = query.get_mut(entities.b2) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
    if keys.just_pressed(KeyCode::E) {
        if let Ok(mut visibility) = query.get_mut(entities.b2) {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

fn on_exit(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}