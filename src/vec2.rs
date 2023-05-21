use std::ops::{Add, Sub, Div, Mul, AddAssign, DivAssign, MulAssign, SubAssign};

// The number trait

pub trait Number: 
    PartialEq +
    Clone +
    Copy +
    Sized + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Div<Output = Self> + 
    Mul<Output = Self> + 
    AddAssign + 
    SubAssign + 
    DivAssign + 
    MulAssign {}

macro_rules! impl_number {
    ($($t:ty),+) => {
        $(impl Number for $t {})*
    };
}

impl_number!(usize, f32, f64, u32, u64, u128, i32, i64, i128);

// Vec Struct 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T: Number>  {
    pub x: T,
    pub y: T
}

macro_rules! impl_vec2_op {
    ($trait:ident, $func:ident) => {
        impl<T: Number> $trait for Vec2<T> {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                Vec2 {
                    x: self.x.$func(rhs.x),
                    y: self.y.$func(rhs.y)
                }
            }
        }

        impl<T:Number> $trait<T> for Vec2<T> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                Vec2 {
                    x: self.x.$func(rhs),
                    y: self.y.$func(rhs)
                }
            }
        }
    };
}

impl_vec2_op!(Add, add);
impl_vec2_op!(Sub, sub);
impl_vec2_op!(Div, div);
impl_vec2_op!(Mul, mul);

macro_rules! impl_vec2_op_assign {
    ($trait:ident, $func:ident) => {        
        impl<T: Number> $trait for Vec2<T> {
            fn $func(&mut self, rhs: Self) {
                self.x.$func(rhs.x);
                self.y.$func(rhs.y);
            }
        }

        impl<T: Number> $trait<T> for Vec2<T> {
            fn $func(&mut self, rhs: T) {
                self.x.$func(rhs);
                self.y.$func(rhs);
            }
        }
    };
}

impl_vec2_op_assign!(AddAssign, add_assign);
impl_vec2_op_assign!(SubAssign, sub_assign);
impl_vec2_op_assign!(DivAssign, div_assign);
impl_vec2_op_assign!(MulAssign, mul_assign);

impl<T: Number> From<T> for Vec2<T> {
    fn from(val: T) -> Self {
        Vec2 {
            x: val,
            y: val,
        }
    }
}
