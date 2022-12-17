use std::f64::consts::PI;
use std::ops::Mul;
use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use std::fs::File;
use rayon::prelude::*;
use ndarray::prelude::*;
use std::include_bytes;

fn main() {
    create_images();
}

fn create_images() {

    fn read_image() -> Array2<f32> {
        let bytes = include_bytes!("heart-500-2.png");
        let buffer = image::load_from_memory(bytes).unwrap().to_luma8();
        let w = buffer.width() as usize;
        let h: usize = buffer.height() as usize;
        let pixel = buffer.iter().map(|x| *x as f32).collect::<Vec<f32>>();
        return Array2::from_shape_vec((w, h), pixel).unwrap();
    }

    let n = 10;
    let _pattern = read_image();


    let r: Vec<i16> = std::ops::Range { start: 0, end: n }.into_iter().collect();
    r.par_iter().for_each(|i| draw(&_pattern, i, n));
}

struct Pos {
    x: f64,
    y: f64,
}

fn ran_pos(pattern: &Array2<f32>) -> Pos {
    let w = pattern.dim().0 as i64;
    let h = pattern.dim().1 as i64;

    let x = rand::random::<f64>() * w as f64;
    let y = rand::random::<f64>() * h as f64;
    return Pos {x, y};
}


fn draw(pattern: &Array2<f32>, index: &i16, max_index: i16) {
    let x = *index as f64 / max_index as f64;
    let y = f(x);
    let n: i32 = (100.0 + y * 1000.0) as i32;
    let w = pattern.dim().0 as i32;
    let h = pattern.dim().1 as i32;

    let surface = ImageSurface::create(Format::Rgb24, w, h).expect("Can't create surface");
    let context = Context::new(&surface).expect("Could not create context");

    println!("index:{index:10}");
    println!("    x:{x:10.5}");
    println!("    y:{y:10.5}");
    println!("    n:{n:10}");

    context.set_source_rgb(0.2, 0.2, 0.2);
    context.paint().expect("Could not paint");

    context.set_source_rgb(0.5, 0.5, 0.5);
    context.line_width();
    for _ in 0..n {
        let pos = ran_pos(pattern);
        context.line_to(pos.x, pos.y);
    }
    context.stroke().expect("Could not strike");

    let base_path = "/home/wwagner4/work/rust/out";
    let path = format!("{base_path}/cairo-{index:05}.png");
    let mut file = File::create(&path).expect(&format!("Couldn't create '{path}'"));
    surface.write_to_png(&mut file).expect("Could notrender image");
    println!("Wrote {path}");
}


fn f(x: f64) -> f64 {
    return -(2.0 * PI * x).cos() * 0.5 + 0.5;
}


