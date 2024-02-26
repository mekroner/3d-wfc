use std::f32::consts::PI;

use bevy::math::Vec3;
use bevy::math::Quat;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Forward,  //-Z
    Backward, //Z
    Left,     // -X
    Right,    // X
    Up,       // Y
    Down,     // -Y
}

impl Dir {
    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Dir::Forward => -Vec3::Z,
            Dir::Backward => Vec3::Z,
            Dir::Left => -Vec3::X,
            Dir::Right => Vec3::X,
            Dir::Up => Vec3::Y,
            Dir::Down => -Vec3::Y,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Dir::Forward => Dir::Backward,
            Dir::Backward => Dir::Forward,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }

    pub fn rotate_y(&self, rotation: Rotation) -> Self {
        match (self, rotation) {
            (Dir::Forward, Rotation::Zero) => Dir::Forward,
            (Dir::Forward, Rotation::Quarter) => Dir::Left,
            (Dir::Forward, Rotation::Half) => Dir::Backward,
            (Dir::Forward, Rotation::ThreeQuarter) => Dir::Right,
            (Dir::Backward, Rotation::Zero) => Dir::Backward,
            (Dir::Backward, Rotation::Quarter) => Dir::Right,
            (Dir::Backward, Rotation::Half) => Dir::Forward,
            (Dir::Backward, Rotation::ThreeQuarter) => Dir::Left,
            (Dir::Left, Rotation::Zero) => Dir::Left,
            (Dir::Left, Rotation::Quarter) => Dir::Backward,
            (Dir::Left, Rotation::Half) => Dir::Right,
            (Dir::Left, Rotation::ThreeQuarter) => Dir::Forward,
            (Dir::Right, Rotation::Zero) => Dir::Right,
            (Dir::Right, Rotation::Quarter) => Dir::Forward,
            (Dir::Right, Rotation::Half) => Dir::Left,
            (Dir::Right, Rotation::ThreeQuarter) => Dir::Backward,
            (Dir::Up, _) => Dir::Up,
            (Dir::Down, _) => Dir::Down,
        }
    }
}

#[derive(EnumIter,Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rotation {
    Zero,
    Quarter,
    Half,
    ThreeQuarter
}

impl Rotation {
    pub fn to_quat(&self) -> Quat {
        match self {
            Rotation::Zero => Quat::from_rotation_y(0.0),
            Rotation::Quarter => Quat::from_rotation_y(-PI/2.0),
            Rotation::Half => Quat::from_rotation_y(-PI),
            Rotation::ThreeQuarter => Quat::from_rotation_y(-3.0*PI/2.0),
        }
    }
}

#[cfg(test)]
mod direction_test {
    use strum::IntoEnumIterator;

    use super::{Dir, Rotation};

    #[test]
    fn test_rotation() {
        for dir in Dir::iter() {
            for rot in Rotation::iter() {
                let expected = rot.to_quat() * dir.to_vec3();
                let actual = dir.rotate_y(rot).to_vec3();
                assert_eq!(expected.round(), actual.round());
            }
        }
    }
}
