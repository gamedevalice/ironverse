use bevy::{prelude::*, ecs::system::EntityCommands};
use crate::systems;

pub trait ToVecString {
    fn to_vec_string(&self) -> Vec<String>;
}
impl ToVecString for Vec<&str> {
    fn to_vec_string(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

pub struct LayoutNode{
    pub tags: Vec<String>,
    pub widget: Widget,
    pub children: Vec<LayoutNode>,
}

pub enum Widget{
    Node(NodeBundle),
    Button(ButtonBundle),
    Text(TextBundle),
    Image(ImageBundle),
    AtlasImage(AtlasImageBundle),
}

#[derive(Component, Clone, Debug)]
pub struct UiTags{
    pub tags: Vec<String>
}
impl UiTags {
    pub fn new(tags: Vec<String>) -> Self {
        UiTags{tags}
    }
}

pub type ThemeFn = fn(asset_server: &Res<AssetServer>, base_widget: Widget, tag: &str) -> Widget;

#[derive(Clone, Resource)]
pub struct UiManager {
    pub theme: ThemeFn
}
impl UiManager {
    pub fn new(theme: ThemeFn) -> Self {
        UiManager{theme}
    }
    pub fn set_layout(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, query: &Query<(Entity, &UiTags)>, layout: LayoutNode) {
        self.despawn_ui(commands, &query);
        self.spawn_ui(commands, layout, &asset_server);
    }
    pub fn set_layout_and_theme(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, query: &Query<(Entity, &UiTags)>, layout: LayoutNode, theme: ThemeFn) {
        self.theme = theme;
        self.set_layout(commands, asset_server, query, layout)
    }
    pub fn despawn_ui(&self, commands: &mut Commands, query: &Query<(Entity, &UiTags)>) {
        query.iter().for_each(|(entity, ui_tags)| {
            if ui_tags.tags.contains(&String::from("root")) {
                commands.entity(entity).insert(systems::DeleteMe{});
            }
        });
    }
    pub fn configure_widget_with_theme(&self, asset_server: &Res<AssetServer>, mut widget: Widget, tags: Vec<String>) -> Widget {
        for tag in &tags {
            widget = (self.theme)(asset_server, widget, &tag);
        }
        widget
    }
    fn spawn_ui(&self, commands: &mut Commands, layout: LayoutNode, asset_server: &Res<AssetServer>) {  
        let mut entity_commands = commands.spawn((NodeBundle{
                style: Style{
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            UiTags::new(vec!["root".to_string()])
        ));
        self.spawn_widget(&mut entity_commands, layout, asset_server);
    }
    fn spawn_widget(&self, entity_commands: &mut EntityCommands, layout: LayoutNode, asset_server: &Res<AssetServer>) {
        match layout.widget {
            Widget::Node(node) => 
            entity_commands.with_children(|parent| {
                let widget = self.configure_widget_with_theme(asset_server, Widget::Node(node), layout.tags.clone());
                let bundle = match widget {
                    Widget::Node(node) => Some(node),
                    _ => None,   
                }.unwrap();
                let mut entity_commands = parent.spawn((bundle, UiTags::new(layout.tags.clone())));
                for child in layout.children {
                    self.spawn_widget(&mut entity_commands, child, asset_server);
                }
            }),
            Widget::Button(button) => 
            entity_commands.with_children(|parent| {
                let widget = self.configure_widget_with_theme(asset_server, Widget::Button(button), layout.tags.clone());
                let bundle = match widget {
                    Widget::Button(button) => Some(button),
                    _ => None,   
                }.unwrap();
                let mut entity_commands = parent.spawn((bundle, UiTags::new(layout.tags.clone())));
                for child in layout.children {
                    self.spawn_widget(&mut entity_commands, child, asset_server);
                }
            }),
            Widget::Text(text) => {
                entity_commands.with_children(|parent| {
                    let widget = self.configure_widget_with_theme(asset_server, Widget::Text(text), layout.tags.clone());
                    let bundle = match widget {
                        Widget::Text(text) => Some(text),
                        _ => None,   
                    }.unwrap();
                    let mut entity_commands = parent.spawn((bundle, UiTags::new(layout.tags.clone())));
                    for child in layout.children {
                        self.spawn_widget(&mut entity_commands, child, asset_server);
                    }
                })
            },
            Widget::Image(image) => {
                entity_commands.with_children(|parent| {
                    let widget = self.configure_widget_with_theme(asset_server, Widget::Image(image), layout.tags.clone());
                    let bundle = match widget {
                        Widget::Image(image) => Some(image),
                        _ => None,   
                    }.unwrap();
                    let mut entity_commands = parent.spawn((bundle, UiTags::new(layout.tags.clone())));
                    for child in layout.children {
                        self.spawn_widget(&mut entity_commands, child, asset_server);
                    }
                })
            },
            Widget::AtlasImage(atlas_image) => {
                entity_commands.with_children(|parent| {
                    let widget = self.configure_widget_with_theme(asset_server, Widget::AtlasImage(atlas_image), layout.tags.clone());
                    let bundle = match widget {
                        Widget::AtlasImage(atlas_image) => Some(atlas_image),
                        _ => None,   
                    }.unwrap();
                    let mut entity_commands = parent.spawn((bundle, UiTags::new(layout.tags.clone())));
                    for child in layout.children {
                        self.spawn_widget(&mut entity_commands, child, asset_server);
                    }
                })
            },
        };
        
    }
}