use crate::vec2::{ToVec2, Vec2};

pub mod vec2;

fn main() {
    let v1 = 5.to_vec2();

    let v2 = Vec2 {
        x: 1,
        y: 2
    };

    let v3 = v1 + v2;

    println!("{:?} + {:?} = {:?}", v1, v2, v3);
}

