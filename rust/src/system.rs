use crate::mat::{Mat2x4, Mat4x1, Mat4x4};

pub struct System {
  /// Encodes the 4D space state
  core: Mat4x4,
}

impl System {
    pub fn new() -> Self {
        Self {
            core: Mat4x4::identity(),
        }
    }

    /// Update the core tensor `Core <- Rot * Core`
    pub fn apply(&mut self, rot: Mat4x4) {
        self.core = rot * self.core;
    }

    /// Determine each register's `increment signature`
    /// by filtering the two axis that form the only congruent plane to `A * (1 0 0 0)^T + B * (0 1 0 0)^T`
    /// then retrieve their signs.
    pub fn active_plane_signature(&self) -> (i8, i8) {
        // current state
        let t = self.core;
        // filter matrix
        let f = Mat2x4::new(
            1, 0, 0, 0,
           0, 1, 0, 0
        );
        // scanner matrix
        let s = Mat4x1::new(
            1, 
            1, 
            1,
            1
        );
        // Since t is guaranteed to be a modified permutation matrix with entries -1, 0, 1
        // 1. t * s will always return a vector whose entries are exactly the non zero values
        //    of the permuted columns.
        // 2. f filters the first two rows of the resulting 4D vector 
        let ret = f * (t * s);
        (ret[0], ret[1]) 
    }

    pub fn display(&self) {
        for (i, item) in self.core.transpose().iter().enumerate() {
            print!("{}", if i % 4 == 0 && i != 0 { '\n' } else { ' ' });
            print!("{}", if item >= &0 { format!( " {item}") } else { item.to_string() });
        }
    }
}
