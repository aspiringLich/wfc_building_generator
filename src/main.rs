use autodefault::autodefault;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use iyes_loopless::prelude::*;

mod cursor;
use cursor::{cursor_event_tilemap, highlight_tile_test, CursorEvent};

/*
TODO: Test the comments work
FIXME: make not broken
? big question question question something idk
! this is very important
* engineer gaming
coment
*/

#[autodefault]
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Building Generator"),
        })
        // base resources
        .insert_resource(ImageSettings::default_nearest())
        // base events
        .add_event::<CursorEvent>()
        // base plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(OverlayPlugin { font_size: 32.0 })
        // startup system(s)
        .add_startup_system(setup_sys)
        // systems
        .add_system(cursor_event_tilemap)
        .add_system(highlight_tile_test.run_on_event::<CursorEvent>())
        .run();
}

/// size of the tilemap, in tiles
pub const TILEMAP_SIZE: TilemapSize = TilemapSize { x: 8, y: 8 };
/// size of the individual tiles in pixels
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 8.0, y: 8.0 };
/// size of the grid the tiles will be laid out on
pub const GRID_SIZE: TilemapGridSize = TilemapGridSize { x: 8.0, y: 8.0 };

/// Used to help identify our main tilemap
#[derive(Component)]
pub struct MainTilemap;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[autodefault]
fn setup_sys(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands
        .spawn_bundle(Camera2dBundle {
            // ! fyi: magic number
            projection: OrthographicProjection { scale: 0.25 },
        })
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Middle],
        })
        .insert(MainCamera);

    let texture_handle: Handle<Image> = asset_server.load("tile.png");

    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(TILEMAP_SIZE);

    // Spawn the elements of the tilemap.
    for x in 0..TILEMAP_SIZE.x {
        for y in 0..TILEMAP_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: TileColor(Color::WHITE),
                })
                .insert(Name::new("Tile"))
                .id();

            // add under the tilemap entity as a child (to avoid clutter)
            commands
                .entity(tilemap_entity)
                .push_children(&[tile_entity]);
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    // spawn the tilemap
    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: GRID_SIZE,
            size: TILEMAP_SIZE,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size: TILE_SIZE,
            transform: get_centered_transform_2d(&TILEMAP_SIZE, &TILE_SIZE, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Main Tilemap"))
        .insert(MainTilemap);
}
