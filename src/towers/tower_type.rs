#[derive(Eq, PartialEq, Hash, Debug)]
pub enum TowerType {
    Turret,
}

impl TowerType {
    pub const ALL: [Self; 1] = [Self::Turret];

    pub fn define(&self) -> TowerDefinition {
        match self {
            TowerType::Turret => TowerDefinition {
                name: "Turret",
                cost: 10,
                rnage: 10.0,
                scene_path: "turret.glb",
                icon_path: "ui/turret.png",
            },
        }
    }
}

pub struct TowerDefinition {
    pub name: &'static str,
    pub cost: u64,
    pub rnage: f32,
    pub scene_path: &'static str,
    pub icon_path: &'static str,
}
