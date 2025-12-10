
pub struct SecretDial {
    size: u32,
    current: u32,
    zeroes: u32,
}

enum Rotation {
    Left(u32),
    Right(u32),
}

impl From<(char, u32)> for Rotation {
    fn from((direction, steps): (char, u32)) -> Self {
        match direction {
            'L' => Rotation::Left(steps),
            'R' => Rotation::Right(steps),
            _ => panic!("Invalid direction character"),
        }
    }
}

impl SecretDial {
    pub fn new(size: u32, current: u32) -> Self {
        SecretDial {
            size,
            current,
            zeroes: 0,
        }
    }

    pub fn get_current_number(&self) -> u32 {
        self.current
    }

    pub fn rotate(&mut self, rotation_input: String) {
        let direction = rotation_input.chars().next().unwrap();
        let steps: u32 = rotation_input[1..].parse().unwrap();

        let rotation = Rotation::from((direction, steps));

        let old_current: u32 = self.current;
        let zeroes: u32;
        match rotation {
            Rotation::Left(s) => {
                let next: i32 = old_current as i32 - s as i32;
                self.current = (self.size as i32 + next).unsigned_abs() % self.size;
                let max = if old_current == 0 { self.size } else { old_current };
                zeroes =  if s < max { 0 } else { 1 + (s as i32  - old_current as i32).unsigned_abs() /self.size} ;
            }
            Rotation::Right(s) => {
                self.current = (self.size  + old_current + s) % self.size;
                zeroes = if s < (self.size - old_current) { 0 } else { 1 + (s as i32  - old_current as i32).unsigned_abs() /self.size};
            }
        }

        self.zeroes += zeroes;

        println!("From {}, rotated {}{} to {}, zeroes {}/{}", old_current, direction, steps, self.current, zeroes, self.zeroes);
    }

    pub fn get_zeroes(&self) -> u32 {
        self.zeroes
    }

    pub fn rotate_many(&mut self, rotation_inputs: Vec<String>) {
        for rotation_input in rotation_inputs {
            self.rotate(rotation_input);
        }
    }
}
