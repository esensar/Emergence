use crate::utils::Position;
use bevy::prelude::*;
pub struct Structure {}
pub struct Plant {}

pub fn build_plant(position: Position) -> impl Bundle {
    (Structure {}, Plant {}, position)
}
pub struct Fungi {}

pub fn build_fungi(position: Position) -> impl Bundle {
    (Structure {}, Fungi {}, position)
}

pub struct StructuresPlugin;
impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(grow_structures.system())
            .add_system(propagate_structures.system());
    }
}

fn grow_structures(mut commands: Commands) {}

fn propagate_structures(mut commands: Commands) {}