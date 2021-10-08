use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64;3],
}

impl Vec3 {
    pub fn new(e1:f64, e2:f64, e3:f64) -> Self {
        let e = [e1,e2,e3];
        Vec3 { e: e }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        (self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2])
    }

    pub fn len(&self) -> f64 {
        (self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]).sqrt()
    }
    pub fn squared_len(&self) -> f64 {
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn scalar_div(self, scalar:f64) -> Self {
        let x = self[0] / scalar;
        let y = self[1] / scalar;
        let z = self[2] / scalar;
        let e = [x,y,z];
        Vec3{e}
    }
    pub fn scalar_mult(self, scalar:f64) -> Self {
        let x = self[0] * scalar;
        let y = self[1] * scalar;
        let z = self[2] * scalar;
        let e = [x,y,z];
        Vec3{e}
    }
    pub fn scalar_add(self, scalar:f64) -> Self {
        let x = self[0] + scalar;
        let y = self[1] + scalar;
        let z = self[2] + scalar;
        let e = [x,y,z];
        Vec3{e}
    }
    pub fn scalar_neg(self, scalar:f64) -> Self {
        let x = self[0] - scalar;
        let y = self[1] - scalar;
        let z = self[2] - scalar;
        let e = [x,y,z];
        Vec3{e}
    }

    pub fn unit_vector(v: Self) -> Self {
        v.scalar_div(v.len())
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-(self.x()), -(self.y()), -(self.z()))
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}


impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output{
        &self.e[index]
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}
