use bevy::prelude::*;
use super::core::{LayoutNode, Widget, ToVecString};


pub fn node(mut additional_tags: Vec<&str>, children: Vec<LayoutNode>) -> LayoutNode {
    let mut tags = vec!["NodeBundle", "node"];
    tags.append(&mut additional_tags);
    LayoutNode{
        tags: tags.to_vec_string(),
        widget: Widget::Node(NodeBundle::default()),
        children: children,
    }
}

pub fn button(mut additional_tags: Vec<&str>, children: Vec<LayoutNode>) -> LayoutNode {
    let mut tags = vec!["ButtonBundle", "button"];
    tags.append(&mut additional_tags);
    LayoutNode{
        tags: tags.to_vec_string(),
        widget: Widget::Button(ButtonBundle::default()),
        children: children,
    }
}

pub fn text(mut additional_tags: Vec<&str>, text: &str, children: Vec<LayoutNode>) -> LayoutNode {
    let mut tags = vec!["TextBundle", "text"];
    tags.append(&mut additional_tags);
    LayoutNode{
        tags: tags.to_vec_string(),
        widget: Widget::Text(TextBundle::from_section(text, TextStyle::default())),
        children: children,
    }
}

pub fn image(mut additional_tags: Vec<&str>, image: Handle<Image>, children: Vec<LayoutNode>) -> LayoutNode {
    let mut tags = vec!["ImageBundle", "image"];
    tags.append(&mut additional_tags);
    LayoutNode{
        tags: tags.to_vec_string(),
        widget: Widget::Image(ImageBundle{
            image: UiImage::new(image.clone()),
            ..default()
        }),
        children: children,
    }
}

//TODO: implement image arg
pub fn atlas_image(mut additional_tags: Vec<&str>, texture_atlas_handle: Handle<TextureAtlas>, children: Vec<LayoutNode>) -> LayoutNode {
    let mut tags = vec!["AtlasImageBundle", "image"];
    tags.append(&mut additional_tags);
    LayoutNode{
        tags: tags.to_vec_string(),
        widget: Widget::AtlasImage(AtlasImageBundle{
            texture_atlas: texture_atlas_handle,
            ..default()
        }),
        children: children,
    }
}