use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    
  }
}


#[derive(Component)]
pub struct CreateNormal {

}

impl Default for CreateNormal {
  fn default() -> Self {
    Self {
      
    }
  }
}


