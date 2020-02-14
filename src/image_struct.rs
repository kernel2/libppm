use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::iter::Iterator;
use std::io::Write;


//Pixel representation using a 24bits (6 per color)
#[derive(Clone, Debug, Copy)]
pub struct Pixel{
    r : u8,
    g : u8,
    b : u8,
}

impl Pixel{
    //Return a pixel with a black color
    pub fn init() -> Pixel {
        return Pixel{
            r:0,
            g:0,
            b:0,
        }
    }

    //Return a pixel with the given color 
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        return Pixel{
            r:red,
            g:green,
            b:blue,
        }
    }

    //Return a string containing the color representation of the pixel
    pub fn display(&self)-> String{
        return format!("{} {} {} ", self.r, self.g,self.b);
    }

    //Invert each color of the pixel
    pub fn invert(&mut self){
        self.r = 255-&self.r;
        self.g = 255-&self.g;
        self.b = 255-&self.b;
    }

    //Convert the pixel into a greyscale
    pub fn greyscale(&mut self){
        let average : u16 = (self.r as u16+self.g as u16+self.b as u16)/3;
        self.r = average as u8;
        self.g = average as u8;
        self.b = average as u8;
    }
}

//Check if the pixel is equal to the one passed in the function parameter
impl PartialEq for Pixel{
    fn eq(&self, other : &Pixel) -> bool{
        self.r == other.r &&
        self.b == other.b &&
        self.g == other.g
    }
}

//Image representation with a vector of pixel, a height & the width
pub struct Image {
    height : usize,
    width : usize,
    pub pixels : Vec<Pixel>,
}

impl Image {
    //Initialize the pixel with the height & the width
    pub fn new(h : usize, w : usize)-> Image {
        return Image {
                height : h,
                width : w,
                pixels : Vec::new()
        }
    }

    //Initialize the pixel with a ppm file
    pub fn new_with_file(filename: &Path) -> Option<Image> {
        //check if the path is a file & if it is a ppm file
        if filename.is_file() && filename.extension().unwrap()=="ppm" {
            let mut init : bool = false;
            let file = match File::open(&filename) {
                Ok(file) => file,
                _ => return None,
            };
            let mut img : Image = Image {
                height : 0,
                width : 0,
                pixels : Vec::new()
            };
            //Initialize the buffer
            let buf_reader = BufReader::new(file);
            let mut h : usize = 0;
            let mut w : usize = 0;
            //Read the file line by line
            for line in buf_reader.lines() {
                let l = line.unwrap();
                //Check if the line is not a comment
                if get_chars_at_index(&l, 0)!='#'{
                    let str_list = l.split_whitespace();
                    let vec = str_list.collect::<Vec<&str>>();
                    //when parsing through the each line, there is 3 possibiliy 
                    //First, there is one element on the line
                    //Second, there is two element
                    //Lastly, there is more than two element (at least 3 to a 3 multiple)
                    match vec.len() {
                        //If there is one element, the string is either the format string of the ppm file, or the string representing the maximum value of a pixel
                        1 => {
                            if get_chars_at_index(&String::from(vec[0]), 0)=='P' {
                            }else {
                                if usize::from_str(vec[0]).unwrap()>255 {
                                    return None;
                                }
                            }
                        },
                        //If there is two element it is the height & the width of the image
                        2 => {
                            h = usize::from_str(vec[1]).unwrap();
                            w = usize::from_str(vec[0]).unwrap();

                            img = Image {
                                height : h,
                                width : w,
                                pixels : Vec::new()
                            };
                            init = true;
                        },
                        //If there is more, we create each pixel with by reading the element of the line in a group of 3
                        _ => {
                            if init == true {
                                for x in (0..vec.len()).step_by(3) {
                                    let r : u8 = u8::from_str(vec[x as usize]).unwrap();
                                    let g : u8 = u8::from_str(vec[x+1 as usize]).unwrap();
                                    let b : u8 = u8::from_str(vec[x+2 as usize]).unwrap();
                                    
                                    let pix : Pixel = Pixel::new(r,g,b);
                                    img.pixels.push(pix);
                                    
                                    }
                            }else{
                                return None;
                            }
                        }

                    }
                }
            }
            return Some(img);
        }
        else {
            return None;
        }
    }
        
    pub fn save(&self, filename: &Path){
        if filename.extension().unwrap()!="ppm" {
            panic!("Wront extension for the file !");
        }
        let format : String = String::from("P3 \n");
        let dimension : String = format!("{} {} \n", &self.width, &self.height);
        let max_pix_color_value : String = String::from("255 \n");
        let mut file = match File::create(&filename) {
            Err(e) => panic!("couldn't create file : {}", e),
            Ok(file) => file,
        };
        
        file.write_all(format.as_bytes());
        file.write_all(dimension.as_bytes());
        file.write_all(max_pix_color_value.as_bytes());
        
        for i in 0..self.height {
            for j in 0..self.width {
                file.write_all(self.get_pixel(i as usize, j as usize).display().as_bytes());
            }
            file.write_all(b"\n");
        }
    }

    pub fn to_string(&self){
        for i in 0..self.height{
            for j in 0..self.width{
                print!("{:?} - ", self.get_pixel(i, j).display());
            }
            println!("");
        }
    }

    pub fn get_pixel(&self, x : usize, y : usize) -> Pixel{
        let index : usize = self.width*x+y;
        return self.pixels[index];
    }

    pub fn greyscale(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].greyscale();
        }
    }

    pub fn invert(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].invert();
        }
    }

}

fn get_chars_at_index(my_string : &String, index :usize) -> char{
    match my_string.chars().nth(index) {
        Some(c) => return c,
        None => panic!("No character at index : {}", index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pixel_inversion() {
        let mut pixel_a : Pixel = Pixel::new(0, 0, 0);
        let pixel_b : Pixel = Pixel::new(255, 255, 255);

        pixel_a.invert();

        assert!(pixel_a.eq(&pixel_b));
    }

    #[test]
    fn greyscale_test() {
        let mut pixel_a : Pixel = Pixel::new(255, 255, 0);
        let pixel_b : Pixel = Pixel::new(170, 170, 170);

        pixel_a.greyscale();

        assert!(pixel_a.eq(&pixel_b));
    }
}
