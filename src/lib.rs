pub mod vec2;

#[cfg(test)]
mod tests {
    use crate::vec2::Vec2;

    #[test]
    fn it_works() {
        // 5, 5
        let v1: Vec2<i32> = 5.into();

        // 1, 2
        let v2 = Vec2 {
            x: 1,
            y: 2
        };

        // 6, 7
        let mut v3 = v1 + v2;

        // 12, 14
        v3 *= Vec2::from(2);

        assert_eq!(v3, Vec2 {
            x: 12,
            y: 14
        });
    }
}

