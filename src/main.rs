use projectRust::image_struct::Image;
use std::path::Path;

fn main(){
    let path_ppm = Path::new("img_test.ppm");
    let mut img = Image::new_with_file(path_ppm).unwrap();
    //let mut img2 = Image::new_with_file(path_ppm).unwrap();
    
    

    img.to_string();
    /*
    println!("invert the image");
    img.invert();
    img.to_string();
    println!("greyscale the image");
    img2.greyscale();
    img2.to_string();
    */
}