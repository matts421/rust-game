use image::{GenericImageView, ImageBuffer, RgbaImage};

fn pad_sprites(
    input_path: &str,
    sprite_width: u32, sprite_height: u32,
    padding_x: u32, padding_y: u32
) {
    let img = image::open(input_path).expect("Failed to open image");
    let (img_width, img_height) = img.dimensions();

    let sprites_per_row = img_width / sprite_width;
    let sprites_per_col = img_height / sprite_height;

    let new_width = sprites_per_row * (sprite_width + padding_x) - padding_x;
    let new_height = sprites_per_col * (sprite_height + padding_y) - padding_y;

    let mut new_img: RgbaImage = ImageBuffer::new(new_width, new_height);

    for y in 0..sprites_per_col {
        for x in 0..sprites_per_row {
            let sprite = img.view(
                x * sprite_width,
                y * sprite_height,
                sprite_width,
                sprite_height,
            );

            let dest_x = x * (sprite_width + padding_x);
            let dest_y = y * (sprite_height + padding_y);

            image::imageops::overlay(&mut new_img, &sprite.to_image(), dest_x.into(), dest_y.into());
        }
    }

    new_img.save(input_path).expect("Failed to save new image");
}

fn main() {
    pad_sprites(
        "assets/textures/tileset.png",
        16, 16, 1, 1
    );
}