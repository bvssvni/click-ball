use piston::*;

use {
    Pos,
};

pub struct App {
    pub mouse: Pos,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl App {
    pub fn render_update(&mut self, args: &RenderArgs) {
        self.screen_width = args.width;
        self.screen_height = args.height;
    }

    pub fn mouse_move_update(&mut self, args: &MouseMoveArgs) {
        self.mouse = Pos {
            x: args.x,
            y: args.y,
        };
    }
}
