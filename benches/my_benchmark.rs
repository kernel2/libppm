use criterion::{criterion_group, criterion_main, Criterion};
use projectRust::image_struct::Pixel;
use projectRust::image_struct::Image;
use std::path::Path;




fn greyscale_test() {
    let mut pixel_a : Pixel = Pixel::new(255, 255, 0);

    pixel_a.greyscale();

   
}

pub fn pixel_inversion() {
    let mut pixel_a : Pixel = Pixel::new(0, 0, 0);

    pixel_a.invert();

}

fn create_image(){
    let path_ppm = Path::new("test.ppm");
    let img = Image::new_with_file(path_ppm).unwrap();

}

fn invert_image(img : &mut Image){
    img.invert();
}

fn greyscale_image(img : &mut Image){
    img.greyscale();

}


//##########################FONCTION BENCHMARK #############################################//



fn greyscale_pixel_benchmark(c: &mut Criterion) {
    c.bench_function("grey_pixel_scale", |b| b.iter(|| greyscale_test()));
}

fn inversion_pixel_benchmark(c: &mut Criterion) {
    c.bench_function("pixel_image_inversion", |b| b.iter(|| pixel_inversion()));
}

fn create_image_benchmark(c: &mut Criterion) {
    c.bench_function("create image", |b| b.iter(|| create_image()));
}

fn invert_image_benchmark(c: &mut Criterion) {
    let path_ppm = Path::new("test.ppm");
    let mut img = Image::new_with_file(path_ppm).unwrap();
    c.bench_function("invert image", |b| b.iter(|| invert_image(&mut img)));
}

fn greyscale_image_benchmark(c: &mut Criterion) {
    let path_ppm = Path::new("test.ppm");
    let mut img = Image::new_with_file(path_ppm).unwrap();
    c.bench_function("greyscale image", |b| b.iter(|| greyscale_image(&mut img)));
}


criterion_group!(benches,
    greyscale_pixel_benchmark,
    inversion_pixel_benchmark,
    create_image_benchmark,
    invert_image_benchmark,
    greyscale_image_benchmark
);

criterion_main!(benches);