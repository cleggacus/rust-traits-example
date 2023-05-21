use std::ops::Add;

// Vec Struct 

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> 
where
    T: Add<Output = T> + Copy + Clone
{
    pub x: T,
    pub y: T
}

// Impliment Add

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Copy + Clone
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

// To Vec Trait

pub trait ToVec2<T> 
where
    T: Add<Output = T> + Copy + Clone
{
    fn to_vec2(&self) -> Vec2<T>;
}

// Impliment To Vec Trait

impl ToVec2<i32> for i32 {
    fn to_vec2(&self) -> Vec2<i32> {
        return Vec2 {
            x: *self,
            y: *self,
        }
    }
}
