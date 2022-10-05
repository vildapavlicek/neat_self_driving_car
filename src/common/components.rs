use bevy::prelude::Component;

/// Marker to indicate that something is `Last` in whatever context
#[derive(Component)]
pub struct Last;

/// This indicates that if we collide with given entity, something should happen
/// ie entities without this marker should be passable (ie you can walk / drive through them without consequences)
#[derive(Component)]
pub struct CanCollide;

/// This marks entity / sprite that camera should be focused on and follow
/// There should always be one and only entity that we have focused
#[derive(Component)]
pub struct CameraFocus;

/// Indicating that the entity has been damaged in any way
/// should be mostly indicator that car has crashed into something
#[derive(Component)]
pub struct Damaged;
