use bevy::prelude::*;

use crate::core::{UiTags, Widget, UiManager};

#[derive(Component, Clone, Debug)]
pub struct DeleteMe{}

pub fn delete_marked_ui(mut commands: Commands, mut delete_query: Query<Entity, (With<DeleteMe>, With<UiTags>)>) {
    for entity in &mut delete_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_manager: Res<UiManager>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &UiTags,
        ),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (entity, interaction, ui_tags) in &mut interaction_query {
        let tags = match *interaction {
            Interaction::None => ui_tags.tags.clone(),
            Interaction::Hovered => append_suffixed_tags(ui_tags.tags.clone(), ":hovered"),
            Interaction::Pressed => append_suffixed_tags(ui_tags.tags.clone(), ":pressed")
        };
        let widget = ui_manager.configure_widget_with_theme(&asset_server, Widget::Button(ButtonBundle::default()), tags);
        let bundle = match widget {
            Widget::Button(button) => Some(button),
            _ => None,   
        }.unwrap();
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).insert(bundle);
        }
    }
}

fn append_suffixed_tags(mut tags: Vec<String>, suffix: &str) -> Vec<String> {
    let mut suffixed_tags = tags.clone();
    suffixed_tags = suffixed_tags.iter().map(|tag| tag.to_owned() + suffix).collect::<Vec<String>>();
    tags.append(&mut suffixed_tags);
    tags
}
