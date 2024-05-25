use bevy::{prelude::*, window::PrimaryWindow};

pub mod game;

pub const HELLO_WORLD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.border = UiRect::all(Val::Px(1.));
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(50.0);
    style.column_gap = Val::Px(50.0);
    style
};


pub fn spawn_hello_world(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("Error getting current window");
    commands.spawn(
        Camera2dBundle {
            // transform: Transform::from_xyz(window.width()/2., window.height()/2., 0.0),
            ..default()
        });

    commands.spawn(
        NodeBundle {
            style: HELLO_WORLD_STYLE,
            border_color: Color::RED.into(),
            ..default()
        }).with_children(|parent|{
            parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Hello World!",
                            get_hello_textstyle(&asset_server),
                        )
                    ],
                    ..default()
                },
                ..default()
            });
        }
    );
}

pub fn get_hello_textstyle(
    asset_server: &Res<AssetServer>,
) -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::WHITE,
        ..default()
    }
}