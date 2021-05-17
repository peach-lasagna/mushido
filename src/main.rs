// #![windows_subsystem = "windows"]

use macroquad::prelude::*;
mod player;
mod resources;

use macroquad::{
    // audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound},
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene::{self, Handle},
    }
};

fn conf() -> Conf{
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: true,
        ..Default::default()
}}

fn get_texture_size(texture: Texture2D) -> (f32, f32){
    (texture.width(), texture.height())
}

#[macroquad::main(conf)]
async fn main(){
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let InternalGlContext { quad_context: ctx, ..} = unsafe { get_internal_gl() };

    ctx.show_mouse(false);
    ctx.set_cursor_grab(true); 
    
    let resources_loading = start_coroutine(async move {
        let resources = resources::Resources::new().await.unwrap();
        storage::store(resources);
    });
    while resources_loading.is_done() == false {
        clear_background(BLACK);
        draw_text(
            &format!(
                "Loading resources {}",
                ".".repeat(((get_time() * 2.0) as usize) % 4)
            ),
            x - 160.0,
            y,
            40.,
            BLACK,
        );
        next_frame().await;
    }

    let (c_w, c_h) = get_texture_size(cursor_texture);


    loop {
        clear_background(WHITE);
        let m_pos = mouse_position();
        if is_key_down(KeyCode::Q) {
            break;
        }
        let (cur_x, cur_y) =  m_pos;

        // draw_texture(cursor_texture, cur_x-c_w / 2.0, cur_y-c_h / 2.0, BLACK);
        // draw_texture(hero_texture, x, y, BLACK);
        // draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}

