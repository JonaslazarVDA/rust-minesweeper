use std::default;

use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;


struct Cell {
    is_mine: bool,
    is_covered: bool,
    is_flagged: bool,
    value: u8,
//    texture: Handle<Image>,
}

//impl Cell {
//    fn select_texture(&mut self) {
impl Cell {
    fn get_texture(&self) -> usize {
        if self.is_flagged {10}
        else if self.is_covered {9}
        else {self.value as usize}
    }
}


impl Default for Cell {
    fn default() -> Self {
        Cell {
            is_mine: false,
            is_covered: true,
            is_flagged: false,
            value: 0,
        }
    }
}


impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell{ is_mine: self.is_mine, is_covered: self.is_covered, is_flagged: self.is_flagged, value: self.value }
    }
}


#[derive(Resource, Default)]
struct Game {
    board: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Game {
    fn init(&mut self, x: usize, y: usize) {
        self.width = x;
        self.height = y;
        self.board = vec![Cell{ ..default() }; x*y];
    }
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(texture_atlas(
            tile_size_x = 16.,
            tile_size_y = 16.,
            columns = 1,
            rows = 13,
            padding_x = 0.,
            padding_y = 0.,
        ))]
    #[asset(path = "assets.png")]
    cell_textures: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_assets: Res<ImageAssets>,
    mut game: ResMut<Game>,
    ) {
    game.init(10, 10);
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {  // screen
            style: Style {
                size: Size::width(Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {  // playing field
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect {
                        left: Val::Px(1.),
                        top: Val::Px(1.),
                        right: Val::Px(1.),
                        bottom: Val::Px(1.),
                    },
                    gap: Size::all(Val::Px(1.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            })
            .with_children(|parent| {

                for row in 0..game.width {

                parent.spawn(NodeBundle {  // rows
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        gap: Size::all(Val::Px(1.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {  // cells

                    for cell in 0..game.height {

                    parent.spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(game.board[row*game.width+cell].get_texture()),
                        texture_atlas: image_assets.cell_textures.clone(),
                        ..default()
                    }); }
                }); }
            })
            .insert(RelativeCursorPosition::default());
        });
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
        )
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
        .init_resource::<Game>()
        .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
//        .add_system(mouse_click_system)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}


fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
    ) {
    let relative_cursor_position = relative_cursor_position_query.single();
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(relative_cursor_position) = relative_cursor_position.normalized {
            println!("x: {}, y: {}", relative_cursor_position.x, relative_cursor_position.y);
        }
    }
}
