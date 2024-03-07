use image::{DynamicImage, GenericImageView, RgbaImage};

pub fn adjust(in_image: &DynamicImage) -> RgbaImage {
  // pega as dimensões da imagem
  let (width, height) = in_image.dimensions();
  // cria a imagem vazia
  let mut out_image = RgbaImage::new(width, height);
  // percorre pelo tamanho de altura quanto largura
  for y in 0..height {
    // para cada linha horizontal da imagem, irá pegar o pixel correspondente
    for x in 0..width {
      let mut pixel = in_image.get_pixel(x, y);
      // println!("{:?}", pixel);
      // loop para cada cor - RGB, excluindo o alpha
      for p in 0..3 {
        // converte o pixel
        pixel[p] = 255 as u8 - pixel[p];
        println!("{:?}", pixel[p]);
      }
      // adiciona o pixel convertido na imagem
      out_image.put_pixel(x, y, pixel);
    }
  }
  out_image
}