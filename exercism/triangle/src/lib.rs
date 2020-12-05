use std::collections::HashMap;

pub struct Triangle {
    sides: HashMap<u64, usize>
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|side|*side == 0) {
            return None;
        }

        let mut sorted_sides = Vec::from(sides);
        sorted_sides.sort();
        
        let two_sides = &sorted_sides[..2];
        if two_sides.iter().fold(0,|a,b|a+b) < *sorted_sides.last().unwrap() {
            return None;
        }

        let mut map: HashMap<u64, usize> = HashMap::default();
        for side in sides.iter() {
            *map.entry(*side).or_insert(0) += 1
        }
        
        Some(Triangle { sides: map })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.keys().len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.keys().len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.keys().len() == 2
    }
}
