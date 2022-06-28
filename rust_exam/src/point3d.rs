use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }
}

// impl Clone for Point3D {
//     fn clone(&self) -> Self {
//         Point3D::new(self.x, self.y, self.z)
//     }
//     fn clone_from(&mut self, source: &Self) {
//         self.x = source.x;
//         self.y = source.y;
//         self.z = source.z;
//     }
// }

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

// impl std::fmt::Debug for Point3D {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({},{},{})", self.x, self.y, self.z)
//     }
// }

impl AddAssign for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Add for Point3D {
    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
    type Output = Self;
}
