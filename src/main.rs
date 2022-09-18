use autodefault::autodefault;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

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
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TilemapPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_startup_system(setup_sys)
        .run();
}

#[autodefault]
fn setup_sys(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Middle],
        });

    let texture_handle: Handle<Image> = asset_server.load("tile.png");

    let tilemap_size = TilemapSize { x: 8, y: 8 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::fill_tilemap.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = TilemapGridSize { x: 8.0, y: 8.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: get_centered_transform_2d(&tilemap_size, &tile_size, 0.0),
            ..Default::default()
        });
}
