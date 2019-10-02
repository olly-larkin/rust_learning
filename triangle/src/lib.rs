pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let invalid = |t: [u64; 3]| {t[0] + t[1] < t[2] || t[0] + t[2] < t[1] || t[1] + t[2] < t[0]};
        if sides.contains(&0) || invalid(sides) {
            None
        } else {
            Some(Triangle{ sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[0] != self.sides[2] && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[0] == self.sides[2] || self.sides[1] == self.sides[2]
    }
}
