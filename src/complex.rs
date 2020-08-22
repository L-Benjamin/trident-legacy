use std::fmt;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Copy, PartialEq, Clone, Default)]
pub struct c64(f32, f32);

unsafe impl ocl::traits::OclPrm for c64 {}

pub const EPSILON: f32 = 1e-7f32;

impl c64 {
    pub const ZERO: c64 = c64::new(0f32, 0f32);
    pub const ONE: c64 = c64::new(1f32, 0f32);

    #[inline]
    pub const fn new(re: f32, im: f32) -> c64 {
        c64(re, im)
    }

    #[inline]
    pub fn conjugate(self) -> c64 {
        c64(self.0, -self.1)
    }

    #[inline]
    pub fn norm_sqr(self) -> f32 {
        self.0*self.0 + self.1*self.1
    }

    #[inline]
    pub fn approx_eq(self, rhs: c64) -> bool {
        (self.0 - rhs.0).abs() < EPSILON && (self.1 - rhs.1).abs() < EPSILON
    }
}

impl fmt::Debug for c64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:+?}i", self.0, self.1)
    }
}

impl std::ops::Add<c64> for c64 {
    type Output = c64;

    fn add(self, rhs: c64) -> c64 {
        c64(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<c64> for c64 {
    type Output = c64;

    fn sub(self, rhs: c64) -> c64 {
        c64(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<c64> for c64 {
    type Output = c64;

    fn mul(self, rhs: c64) -> c64 {
        c64(
            self.0*rhs.0 - self.1*rhs.1,
            self.0*rhs.1 + self.1*rhs.0,
        )
    }
}