use image;
use std::path::Path;

mod process;

use process::negative;

fn main() {
    println!("Imagem gerada");
    let img = image::open("alien.jpg").unwrap(); // abre a imagem
    let neg_image = negative::adjust(&img);
    let path = Path::new("alientest.jpg"); // cria o path da imagem
    neg_image.save(path).unwrap(); // salva a imagem no path
}
