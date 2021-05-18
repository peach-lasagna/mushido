use macroquad::prelude::*;

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!( Resources {
    player: Texture2D,
    cursor: Texture2D,
});
// async fn load_t(path: &str) -> Result<Texture2D, macroquad::prelude::FileError> {
//     let texture = load_texture(path).await?;
//     texture.set_filter(FilterMode::Nearest);
//     texture
// }

impl Resources{
    pub async  fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let player = load_texture("assets/mushroom-sheet.png").await?;
        player.set_filter(FilterMode::Nearest);
        
        let cursor = load_texture("assets/cursor.png").await?;
        // cursor_texture.set_filter(FilterMode::Nearest);

        Ok(Resources{
            player, cursor
        })
    }
}

