use bevy::app::App;

pub mod one;

pub fn plugin(app: &mut App) {
    one::plugin(app);
}
