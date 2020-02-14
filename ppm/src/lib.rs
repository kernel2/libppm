
pub mod pixel;
pub mod image;

pub use crate::pixel::pixel as pixel_mod;
pub use crate::image::image as image_mod;

#[no_mangle]
pub extern fn toSeeIfPixelWorks() {
    let mut p = pixel_mod::Pixel::new(53, 66, 27);
    println!("Pixel: {}\n",p.display() );
    let p2 =  pixel_mod::Pixel::clone(&p);
    println!("Cloned pixel: {}\n",p2.display());
    p.invert();
    println!("Inversion: {}\n",p.display());
    p.grayscale();
    println!("Grayscaled: {}\n",p.display());
    
    
}


