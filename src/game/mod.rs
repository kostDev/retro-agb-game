use agb::{
    display::object::Object,
    fixnum::Vector2D,
    input::Tri
};

pub struct Player {
    pub pos: Vector2D<i32>,
    pub speed: i32, // vel: Vector2D<i32>,
    pub obj: Object,
}

impl Player {
    pub fn new(pos: Vector2D<i32>, speed: i32, obj: Object) -> Player {
        Self { pos, speed, obj }
    }

    pub fn handle_input(&mut self, x_tri: Tri) {
        match x_tri {
            Tri::Positive => {
                self.pos.x = (self.pos.x + self.speed)
                    .clamp(0, agb::display::WIDTH - 16);
            },
            Tri::Negative => {
                self.pos.x = (self.pos.x - self.speed)
                    .clamp(0, agb::display::WIDTH - 8);
            },
            Tri::Zero => {},
        }

        // match y_tri {
        //     Tri::Positive => {
        //         self.pos.y = (self.pos.y + self.speed)
        //             .clamp(0, agb::display::HEIGHT - 16);
        //     },
        //     Tri::Negative => {
        //         self.pos.y = (self.pos.y - self.speed)
        //             .clamp(0, agb::display::HEIGHT - 8);
        //     },
        //     Tri::Zero => {},
        // }
    }

    pub fn update(&mut self) {
        // update player object with pos value as Vector2D value
        // lib: Set the position of the ball to match our new calculated position
        self.obj.set_pos(self.pos);
    }
}