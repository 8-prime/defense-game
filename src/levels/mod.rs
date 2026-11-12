use bevy::app::App;

pub mod one;
pub mod path;
pub mod phase;

pub fn plugin(app: &mut App) {
    one::plugin(app);
    phase::plugin(app);
}
