use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct MaybeEntity {
    pub entity : Option<Entity>
}