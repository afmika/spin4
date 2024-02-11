use nalgebra::SMatrix;
use anyhow::{bail, Result};

pub type Mat4x4 = SMatrix<i8, 4, 4>;

#[derive(Debug, Clone)]
pub enum Axis {
    X, Y, Z, W
}

/// `pi/2` or `-pi/2` 4D rotation relative to a plane generated by `A * u + B * v `
pub fn rotPlane(u: Axis, v: Axis, reversed: bool) -> Result<Mat4x4> {
    let (sint, cost) = match reversed {
        true => (1, 0),
        false => (-1, 0)
    };

    match (u.clone(), v.clone()) {
        // Rxy
        (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Ok(Mat4x4::new(
            cost, -sint, 0, 0,
            sint, cost, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1
        )),
        // Rxz
        (Axis::X, Axis::Z) | (Axis::Z, Axis::X) => Ok(Mat4x4::new(
            cost, 0, -sint, 0,
            0, 1, 0, 0,
            sint, 0, cost, 0,
            0, 0, 0, 1
        )),
        // Rxw
        (Axis::X, Axis::W)| (Axis::W, Axis::X)  => Ok(Mat4x4::new(
            cost, 0, 0, -sint,
            0, 1, 0, 0,
            0, 0, 1, 0,
            sint, 0, 0,  cost,
        )),
        // Ryz
        (Axis::Y, Axis::Z)| (Axis::Z, Axis::Y)  =>Ok(Mat4x4::new(
            1, 0, 0, 0,
            0, cost, -sint, 0,
            0, sint, cost, 0,
            0, 0, 0, 1
        )),
        // Ryw
        (Axis::Y, Axis::W)| (Axis::W, Axis::Y)  => Ok(Mat4x4::new(
            1, 0, 0, 0,
            0, cost, 0, -sint,
            0, 0, 1, 0,
            0, sint, 0, cost
        )),
        // Rzw
        (Axis::Z, Axis::W)| (Axis::W, Axis::Z)  => Ok(Mat4x4::new(
            1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, cost, -sint,
            0, 0, sint, cost
        )),
        (Axis::X, Axis::X) 
            | (Axis::Y, Axis::Y) 
            | (Axis::Z, Axis::Z)
            | (Axis::W, Axis::W) => bail!(format!("({:?}, {:?}) forms a line", u, v)),
    }
}