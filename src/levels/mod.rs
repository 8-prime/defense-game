use bevy::app::App;

pub mod one;
pub mod path;

pub fn plugin(app: &mut App) {
    one::plugin(app);
}
