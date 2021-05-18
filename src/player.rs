use macroquad::{
    experimental::{
        animation::{AnimatedSprite, Animation},
        collections::storage,
        scene::{self, RefMut},
    },
    color,
    prelude::*,
};
use crate::resources;

pub struct Cursor {
    pub pos: Vec2,
    sprite: AnimatedSprite,
}

// impl Cursor {
//     pub fn new() ->  Self {
//         Cursor {} // TODO
//     }

//     pub fn draw(&mut self){
//         let resources = storage::get::<resources::Resources>();
//         self.pos = mouse_position();
//         draw_texture_ex(
//             resources.cursor,
//             self.pos.x, // TODO
//             self.pos.y,
//             color::BLACK,
//             DrawTextureParams {
//                 source: Some(self.sprite.frame().source_rect),
//                 dest_size: Some(self.sprite.frame().dest_size),
//                 ..Default::default()
//             },
//         );
//     }
// }

// impl scene::Node for Cursor{
//     fn draw(mut node: RefMut<Self>) {
//         node.draw()
//     }
// }


pub struct Player {
    pos: Vec2,
    sprite: AnimatedSprite,
    dead: bool,
}

impl Player {
    pub const SPEED: f32 = 5.0;
    pub const DASH_SPEED: f32 = 15.0;

    pub fn new(pos: Vec2, ) -> Self {
        let mut sprite = AnimatedSprite::new(
            30,
            30,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 1,
                    fps: 1,
                },
                Animation {
                    name: "dash".to_string(),
                    row: 1,
                    frames: 9,
                    fps: 10,
                },
            ],
            true,
        );
        Self { pos, sprite, dead: false }
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn set_animation(&mut self, animation: usize) {
        self.sprite.set_animation(animation);
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::D) {
            self.pos.x += Self::SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.pos.x -= Self::SPEED;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y += Self::SPEED;
        }
        if is_key_down(KeyCode::W) {
            self.pos.y -= Self::SPEED;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.set_animation(1);
            let (x_c, y_c) = mouse_position();// TODO
        } else {
            self.set_animation(0);
        }
    }
    pub fn draw_hero(&mut self) {
        self.sprite.update();
        if self.is_dead() {
            return;
        }
        let resources = storage::get::<resources::Resources>();
        draw_texture_ex(
            resources.player,
            self.pos.x,
            self.pos.y,
            color::BLACK,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                ..Default::default()
            },
        );

    }
}

impl scene::Node for Player {
    fn draw(mut node: RefMut<Self>) {
        // node.sprite.update();
        node.draw_hero();
    }
    fn update(mut node: RefMut<Self>) {
        node.update();
    }
}
