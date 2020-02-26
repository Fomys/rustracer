use crate::raytracer::utils::Vec3;

pub struct MovementPart {
    pub start_frame: usize,
    pub end_frame: usize,
    pub movement: MovementPrimitive,
}

impl MovementPart {
    pub fn is_active(&self, frame: usize) -> bool {
        self.start_frame <= frame && frame < self.end_frame
    }
}

pub struct Movement {
    frame: usize,
    pub movements: Vec<MovementPart>,
}

impl Movement {
    pub fn new(movements: Vec<MovementPart>) -> Self {
        Self {
            frame: 0,
            movements,
        }
    }

    pub fn next_movements(&mut self) -> Vec<MovementPrimitive> {
        let mut to_ret: Vec<MovementPrimitive> = vec![];
        let mut next_frame = self.frame + 1;
        for movement_part in self.movements.iter() {
            if movement_part.is_active(self.frame) {
                to_ret.push(movement_part.movement);
                match movement_part.movement {
                    MovementPrimitive::Cycle(frame) => {
                        next_frame = frame;
                    }
                    _ => {}
                }
            }
        }
        self.frame = next_frame;
        to_ret
    }
}

#[derive(Clone, Copy)]
pub enum MovementPrimitive {
    /// Translation of object
    Translation(Vec3),
    /// Scale object
    Scale(f32),
    /// Continue animation from this frame
    Cycle(usize),
}
