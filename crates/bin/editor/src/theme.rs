use bevy::prelude::*;
use bevy_iron_ui::core::Widget;

pub fn theme(asset_server: &Res<AssetServer>, base_widget: Widget, tag: &str) -> Widget {
    let font = asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf");

    match base_widget {
        Widget::Node(mut node) => {
            match tag {
                "menu" => {
                    node.style = Style {
                        width: Val::Percent(33.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..node.style
                    };
                    node.background_color = BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.5));
                },
                "voxel_edit_mode" => {
                    node.style = Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..node.style
                    };
                },
                "voxel_edit_mode_controls_list" => {
                    node.style = Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        position_type: PositionType::Absolute,
                        ..node.style
                    };
                },
                "voxel_edit_options" => {
                    node.style = Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..node.style
                    };
                },
                "hotbar" => {
                    node.style = Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..node.style
                    };
                },
                "hotbar_item_color" => {
                    node.style = Style {
                        width: Val::Percent(90.0),
                        height: Val::Percent(90.0),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
                        ..node.style
                    };
                    node.border_color = Color::BLACK.into()
                },
                "hotbar_item_color_1" => {
                    node.border_color = Color::GRAY.into()
                },
                "color_picker" => {
                    node.style = Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(16, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(16, 1.0),
                        margin: UiRect::new(Val::Percent(55.0), Val::Percent(0.0), Val::Percent(7.5), Val::Percent(00.0)),
                        ..node.style
                    };
                },
                "color_picker_item_color" => {
                    node.style = Style {
                        width: Val::Percent(90.0),
                        height: Val::Percent(90.0),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
                        ..node.style
                    };
                    node.border_color = Color::BLACK.into()
                },
                "color_edit" => {
                    node.style.flex_direction = FlexDirection::Row;
                    node.style.justify_content = JustifyContent::Center;
                    node.style.align_items = AlignItems::Center;
                    node.style.margin = UiRect::left(Val::Percent(55.0));

                },
                "rgb_edit" => {
                    node.style.flex_direction = FlexDirection::Row;
                },
                "rgb_channel_edit" => {
                    node.style.flex_direction = FlexDirection::Row;
                    node.style.margin = UiRect::all(Val::Px(10.0));
                },
                _ => {}
            }
            return Widget::Node(node);
        },
        Widget::Button(mut button) => {
            match tag {
                "button" => {
                    button.style = Style {
                        width: Val::Px(300.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Px(5.0)), // horizontally center child text
                        justify_content: JustifyContent::Start, // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..button.style
                    };
                    button.border_color = BorderColor(Color::BLACK);
                    button.background_color = Color::rgb(0.15, 0.15, 0.15).into();
                },
                "button:hovered" => {
                    button.border_color = BorderColor(Color::WHITE);
                    button.background_color = Color::rgb(0.25, 0.25, 0.25).into();
                },
                "button:pressed" => {
                    button.border_color = BorderColor(Color::RED);
                    button.background_color = Color::rgb(0.35, 0.75, 0.35).into();
                },
                "hotbar_button" => {
                    button.style = Style {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..button.style
                    };
                },
                "color_picker_button" => {
                    button.style = Style {
                        width: Val::Px(35.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..button.style
                    };
                },
                "+-_button" => {
                    button.style = Style {
                        width: Val::Px(35.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..button.style
                    };
                },
                _ => {}
            }
            return Widget::Button(button);
        },
        Widget::Text(mut text) => {
            match tag {
                "text" => {
                    text.text.sections[0].style = TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    };
                },
                "voxel_edit_mode_controls_text" => {
                    text.text.sections[0].style.font_size = 20.0;
                },
                _ => {}   
            }
            return Widget::Text(text);
        },
        Widget::Image(mut image) => {
            match tag {
                "crosshair" => {
                    image.style.position_type = PositionType::Absolute;
                },
                _ => {}   
            }
            return Widget::Image(image);
        },
        Widget::AtlasImage(atlas_image) => {
            return Widget::AtlasImage(atlas_image);
        },
    }
}