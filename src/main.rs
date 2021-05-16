// #![windows_subsystem = "windows"]

use macroquad::prelude::*;


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
    let speed = 5.0;
    let hero_texture = load_texture("assets/mushroom.png").await.unwrap();
    let cursor_texture = load_texture("assets/cursor.png").await.unwrap();
    let (c_w, c_h) = get_texture_size(cursor_texture);
    let InternalGlContext { quad_context: ctx, ..} = unsafe { get_internal_gl() };

    ctx.show_mouse(false);
    ctx.set_cursor_grab(true); 

    loop {
        clear_background(WHITE);
        let m_pos = mouse_position();
        if is_key_down(KeyCode::Q) {
            break;
        }
        if is_key_down(KeyCode::D) {
            x += speed;
        }
        if is_key_down(KeyCode::A) {
            x -= speed;
        }
        if is_key_down(KeyCode::S) {
            y += speed;
        }
        if is_key_down(KeyCode::W) {
            y -= speed;
        }
        let (cur_x, cur_y) =  m_pos;

        draw_texture(cursor_texture, cur_x-c_w / 2.0, cur_y-c_h / 2.0, BLACK);
        draw_texture(hero_texture, x, y, BLACK);
        // draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}

