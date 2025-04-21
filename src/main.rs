use bevy::prelude::*;
use dotenvy::dotenv;

const PLAYER_SPEED: f32 = 200.0;

fn main() {
    // To explicitly set a graphics backend, create an .env file with WGPU_BACKEND= dx12 (windows), metal (macos), vulkan (linux)
    dotenv().ok();
    
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update, (
                animate_sprite,
                handle_direction,
                movement_system,
                quit
            ))
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

fn quit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/character/tw.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 6, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { curr: 0, first: 0, last: 5, offset: 0 };

    commands.spawn(Camera2d);

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
        Transform::from_scale(Vec3::splat(8.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),

        // Player attributes
        Player,
        Motion { velo: Vec2::ZERO },
    ));
}
