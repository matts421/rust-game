mod map;
mod ui;
mod shared;

use dotenvy::dotenv;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::shared::*;
use crate::map::MapPlugin;
use crate::ui::UIPlugin;

fn main() {
    dotenv().ok();
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            // LogDiagnosticsPlugin::default(), // Adds frame time, FPS and frame count diagnostics.
            // FrameTimeDiagnosticsPlugin::default(), // Adds an entity count diagnostic.
        ))
        .add_plugins(MapPlugin)
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)

        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update, (
                animate_sprite,
                handle_direction,
                movement_system,
                update_camera,
                quit
            ).run_if(in_state(GameState::Playing)))
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize, last: usize,
    curr: usize, offset: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Motion {
    velo: Vec2,
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if indices.curr == indices.last {indices.curr = indices.first;}
                else                            {indices.curr = indices.curr + 1;}
                atlas.index = indices.curr + indices.offset;
            }
        }
    }
}

fn handle_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AnimationIndices, &mut Motion), With<Player>>,
) {
    for (mut indices, mut motion) in &mut query {
        motion.velo = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) {motion.velo.x = -1.0; indices.offset = 6 * 3;}
        if keyboard_input.pressed(KeyCode::KeyD) {motion.velo.x =  1.0; indices.offset = 6 * 2;}
        if keyboard_input.pressed(KeyCode::KeyW) {motion.velo.y =  1.0; indices.offset = 6 * 1;}
        if keyboard_input.pressed(KeyCode::KeyS) {motion.velo.y = -1.0; indices.offset = 6 * 0;}
        motion.velo = PLAYER_SPEED * motion.velo.normalize_or_zero();
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Motion)>) {
    for (mut transform, motion) in &mut query {
        transform.translation.x += motion.velo.x * time.delta_secs();
        transform.translation.y += motion.velo.y * time.delta_secs();
    }
}

fn update_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let mut new_pos = player_transform.translation;
            new_pos.z = camera_transform.translation.z;
            camera_transform.translation = new_pos;
        }
    }
}

fn quit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/character/tw.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE), 6, 8, Some(UVec2 { x: 0, y: 1 }), None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { curr: 0, first: 0, last: 5, offset: 0 };

    commands.spawn((
        Sprite::from_image(asset_server.load("textures/bg.png")),
        Transform {
            translation: Vec3 {x: 0.0, y: 0.0 , z: -1.0},
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(GAME_SCALE)
        }
    ));

    // Create player bundle
    commands.spawn((
        // Sprite attributes
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.curr + animation_indices.offset,
            },
        ),
        Transform {
            translation: Vec3 {x: 0.0, y: 0.0 , z: 0.0},
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(GAME_SCALE)
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),

        // Player attributes
        Player,
        Motion { velo: Vec2::ZERO },
    ));
}
