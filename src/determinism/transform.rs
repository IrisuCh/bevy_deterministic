use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct Size {
    pub x: I32F32,
    pub y: I32F32,
}

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: I32F32::from_num(x),
            y: I32F32::from_num(y),
        }
    }

    pub const fn new_fixed(x: I32F32, y: I32F32) -> Self {
        Self { x, y }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x.to_num::<f32>(), self.y.to_num::<f32>(), 0.0)
    }

    pub fn x_f32(&self) -> f32 {
        self.x.to_num::<f32>()
    }

    pub fn y_f32(&self) -> f32 {
        self.y.to_num::<f32>()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Point {
    pub x: I32F32,
    pub y: I32F32,
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
#[require(GlobalPosition)]
pub struct Position {
    pub x: I32F32,
    pub y: I32F32,
}

impl Position {
    pub fn new(x: I32F32, y: I32F32) -> Self {
        Self { x, y }
    }

    pub fn new_f32(x: f32, y: f32) -> Self {
        Self {
            x: I32F32::from_num(x),
            y: I32F32::from_num(y),
        }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x.to_num::<f32>(), self.y.to_num::<f32>(), 0.0)
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct GlobalPosition {
    x: I32F32,
    y: I32F32,
}

impl GlobalPosition {
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x.to_num::<f32>(), self.y.to_num::<f32>(), 0.0)
    }

    pub fn x(&self) -> I32F32 {
        self.x
    }

    pub fn y(&self) -> I32F32 {
        self.y
    }

    pub fn x_f32(&self) -> f32 {
        self.x.to_num::<f32>()
    }

    pub fn y_f32(&self) -> f32 {
        self.y.to_num::<f32>()
    }
}

pub fn sync_global_positions(query: Query<(&mut GlobalPosition, &Position), Changed<Position>>) {
    for (mut global, local) in query {
        global.x = local.x;
        global.y = local.y;
    }
}

pub fn sync_positions(
    query: Query<(Entity, &Children), Changed<Position>>,
    mut positions: Query<(&mut GlobalPosition, &Position)>,
) {
    for (parent, children) in query {
        let (global_x, global_y) = {
            let global = positions.get(parent).unwrap().0;
            (global.x, global.y)
        };
        for entity in children {
            if let Ok((mut child_global, child_local)) = positions.get_mut(*entity) {
                child_global.x = global_x + child_local.x;
                child_global.y = global_y + child_local.y;
            }
        }
    }
}
