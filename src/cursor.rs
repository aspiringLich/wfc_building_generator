use std::convert::TryInto;

use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_ecs_tilemap::prelude::*;

use crate::{MainCamera, MainTilemap, GRID_SIZE, TILEMAP_SIZE};

/// Cursor events that signal things of note hapenning to do with the cursor
///  - MovedOffTile(TilePos): sent when the mouse moves off a tile
///  - MovedOnTile(TilePos): sent when the mouse moves on a tile
pub enum CursorEvent {
    MovedOffTile(TilePos),
    MovedOnTile(TilePos),
}

/// get the coorosponding tilemap square the cursor is currently hovered over barring all the fancy camera stuff
/// and notify other systems with an event
///
/// shamelessly repurposed from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn cursor_event_tilemap(
    windows: Res<Windows>,
    // camera transform query
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // query the tilemap for its position
    q_tilemap_pos: Query<&Transform, With<MainTilemap>>,
    // local for storing which tile the cursor was on at the previous run of this function, if any
    mut prev_pos: Local<Option<TilePos>>,
    // event queue for CursorEvent
    mut cursor_event: EventWriter<CursorEvent>,
) {
    // get the camera info and transform
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        // where is our mouse in comparison to the bottom left corner of the tilemap
        let tilemap: Vec2 = q_tilemap_pos.single().translation.truncate();
        let rel_to_tilemap: Vec2 = world_pos - tilemap;

        // finally, get the tile were hoverring over (technically)
        let (x, y) = (
            rel_to_tilemap.x / GRID_SIZE.x,
            rel_to_tilemap.y / GRID_SIZE.y,
        );

        // check its valid (if the mouse is actually inside the bounds of the tilemap)
        let valid = x >= 0.0 && y >= 0.0;
        let x = x as u32;
        let y = y as u32;
        let valid = valid && x < TILEMAP_SIZE.x && y < TILEMAP_SIZE.y;

        let tile_pos = TilePos::new(x, y);
        let mut prev_valid = false;
        let mut neq = false;

        // if the prev tile was valid, and its neq to the current, send an event that we moved off
        if let Some(pos) = *prev_pos {
            prev_valid = true;
            neq = pos.x != x || pos.y != y;
            if neq || !valid {
                cursor_event.send(CursorEvent::MovedOffTile(pos));
                // eprintln!("Moved off ({}, {})", pos.x, pos.y);
            }
        }
        // if the current tile is valid, and its neq to the prev, send an event that we moved on
        // also update prev pos
        if valid {
            if neq || !prev_valid {
                cursor_event.send(CursorEvent::MovedOnTile(tile_pos));
                // eprintln!("Moved on ({}, {})", tile_pos.x, tile_pos.y);
            }
            *prev_pos = Some(tile_pos);
        } else {
            *prev_pos = None;
        }
    }
}
