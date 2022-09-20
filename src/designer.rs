use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, ImageButton},
    EguiContext,
};

pub struct BlockType {
    pub color: Color,
    pub name: &'static str,
}

impl BlockType {
    pub const fn new(color: Color, name: &'static str) -> Self {
        BlockType { color, name: name }
    }
}

pub const BLOCKTYPES: [BlockType; 2] = [
    BlockType::new(Color::WHITE, "Blank"),
    BlockType::new(Color::DARK_GRAY, "Wall"),
];

/// a struct to hold useful data pertaining to the function `block_selector_ui`
pub struct BlockSelectorUiState {
    tile: Handle<Image>,
}

impl FromWorld for BlockSelectorUiState {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            tile: asset_server.load("tile.png"),
        }
    }
}

/// for ergonomics while working with egui color
pub trait IntoColor32 {
    fn into_col32(&self) -> Color32;
}

impl IntoColor32 for Color {
    fn into_col32(&self) -> Color32 {
        use Color::*;
        match self {
            Rgba {
                red: r,
                green: g,
                blue: b,
                alpha: _,
            } => Color32::from_rgb((r * 256.) as u8, (g * 256.) as u8, (b * 256.) as u8),
            _ => todo!(),
        }
    }
}

/// ui element:
/// Allows you to choose different blocks in designer mode
pub fn block_selector_ui(
    // duh, necessary for egui
    mut egui_context: ResMut<EguiContext>,
    // stores the image(s) used inside the ui
    ui_state: Local<BlockSelectorUiState>,
) {
    const BUTTON_SIZE: (f32, f32) = (50., 50.);
    let tile = egui_context.add_image(ui_state.tile.as_weak());

    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        for block_type in BLOCKTYPES {
            ui.add(egui::Button::new(block_type.name).fill(block_type.color.into_col32()));
        }
    });
}
