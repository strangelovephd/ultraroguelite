mod object;

use std::boxed::Box;

use tcod::console::*;
use tcod::colors;
use crate::object::{GameObject, NPC, Objects, Player};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 60;

fn handle_keys(root: &mut Root, obj: &mut GameObject) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => obj.move_by(0, -1),
        Key { code: Down, .. } => obj.move_by(0, 1),
        Key { code: Left, .. } => obj.move_by(-1, 0),
        Key { code: Right, .. } => obj.move_by(1, 0),
        _ => {},
    }

    false
}

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("ultrroguelite")
        .init();
    
    tcod::system::set_fps(LIMIT_FPS);

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let p = Player::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = NPC::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);


    let mut objects: Objects = Objects { objs: vec![Box::new(p), Box::new(npc)] };

    while !root.window_closed() {
        for object in &objects.objs {
            object.draw(&mut con);
        }

        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();
        
        for object in &objects.objs {
            object.clear(&mut con);
        }

        con.set_default_foreground(colors::WHITE);
        
        // root.clear();
        
        
        root.wait_for_keypress(true);

        let player = &mut (*objects.objs[0]);

        if handle_keys(&mut root, player) {
            break
        }
    }

}
