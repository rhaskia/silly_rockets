use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("sprites/grass.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_texture(&texture, screen_width() / 2., screen_height() / 2., WHITE);
        next_frame().await
    }
}