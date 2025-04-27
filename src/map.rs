use bevy::prelude::*;
use crate::constants::{GAME_SCALE, TILE_SIZE};

type Map = Vec<Vec<u8>>;

fn create_default_map() -> Map {
    let rows = 10;
    let cols = 10;

    let mut arr: Map = vec![vec![0u8; cols]; rows];
    for i in 1..rows {
        for j in 1..cols {
            if i % 2 == 0 {
                arr[i][j] = 26u8;
            } else {
                arr[i][j] = 22u8;
            }
        }
    }
    arr
}


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/tileset.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE), 16, 4, Some(UVec2 { x: 1, y: 1 }), None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let map = create_default_map();
    for i in 1..map.len() {
        for j in 1..map[i].len() {
            commands.spawn(
                (
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: map[i][j] as usize,
                        },
                    ),
                    Transform {
                        translation: Vec3 {x: (j * 16 * 8) as f32, y: (i * 16 * 8) as f32, z: -0.5},
                        rotation: Quat::IDENTITY,
                        scale: Vec3::splat(GAME_SCALE)
                    },
                )
            );
        }
    }
}

