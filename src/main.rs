use std::f64::consts::PI;
use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use std::fs::File;
use rayon::prelude::*;

fn main() {


    let width = 800;
    let height = 800;
    let n = 50;

    let r: Vec<i16> = std::ops::Range { start: 0, end: n }.into_iter().collect();
    r.par_iter().for_each(|i| draw(width, height, i, n));

}

fn draw(width: i32, height: i32, index: &i16, max_index: i16) {
    let x = *index as f64 / max_index as f64;
    let y = f(x);
    let n: i32 = (2.0 + y * 5000.0) as i32;

    let surface = ImageSurface::create(Format::Rgb24, width, height).expect("Can't create surface");
    let context = Context::new(&surface).expect("Could not create context");

    println!("index:{index:10}");
    println!("    x:{x:10.5}");
    println!("    y:{y:10.5}");
    println!("    n:{n:10}");

    context.set_source_rgb(0.2, 0.2, 0.2);
    context.paint().expect("Could not paint");

    context.set_source_rgb(0.5, 0.5, 0.5);
    context.line_width();
    for _i in 0..n {
        let x = rand::random::<f64>() * width as f64;
        let y = rand::random::<f64>() * height as f64;
        context.line_to(x, y);
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


