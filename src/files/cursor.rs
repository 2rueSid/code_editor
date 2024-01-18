pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Cursor {
    pub fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, max_y: u16) {
        if self.y + 1 >= max_y {
            return;
        }
        self.y += 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }
}
