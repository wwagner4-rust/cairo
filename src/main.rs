use std::f32::consts::PI;
use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;
use std::fs::File;
use rayon::prelude::*;
use ndarray::prelude::*;
use std::include_bytes;

fn main() {

    fn f(x: f32) -> f32 {
        return -(2.0 * PI * x).cos() * 0.5 + 0.5;
    }

    struct Pos {
        x: usize,
        y: usize,
    }

    fn ran_pos(pattern: &Array2<f32>, y: f32) -> Pos {
        fn rand(max: usize) -> usize {
            return (rand::random::<f32>() * max as f32) as usize;
        }

        let mut cnt = 0;
        let w = pattern.dim().0 as usize;
        let h = pattern.dim().1 as usize;
        let top = 50.0 + y * 200.0;
        //println!("top {:10.1}", top);

        let mut x = rand(w);
        let mut y = rand(h);
        let mut pos = Pos { x, y };
        let mut value = pattern[[x, y]];
        //println!("value: {:10.4}", value);
        while !(cnt > 500 || value < top) {
            x = rand(w);
            y = rand(h);
            pos = Pos { x, y };
            value = pattern[[x, y]];
            cnt += 1;
        }
        //println!("value: {:10} {:10.4}", cnt, value);
        return pos;
    }

    fn read_image() -> Array2<f32> {
        let bytes = include_bytes!("heart-500.png");
        let buffer = image::load_from_memory(bytes).unwrap().to_luma8();
        let w = buffer.width() as usize;
        let h: usize = buffer.height() as usize;
        let pixel = buffer.iter().map(|x| *x as f32).collect::<Vec<f32>>();
        return Array2::from_shape_vec((w, h), pixel).unwrap();
    }

    fn draw(pattern: &Array2<f32>, index: &i16, max_index: i16) {
        println!("Started image {index}");

        let x = *index as f32 / max_index as f32;
        let y = f(x);
        let n: i32 = 300;
        let w = pattern.dim().0 as i32;
        let h = pattern.dim().1 as i32;

        let surface = ImageSurface::create(Format::Rgb24, w, h).expect("Can't create surface");
        let context = Context::new(&surface).expect("Could not create context");


        context.set_source_rgb(0.2, 0.2, 0.2);
        context.paint().expect("Could not paint");

        context.set_source_rgb(0.5, 0.5, 0.5);
        context.line_width();
        for _ in 0..n {
            let pos = ran_pos(pattern, y);
            context.line_to(pos.x as f64, pos.y as f64);
        }
        context.stroke().expect("Could not strike");

        let base_path = "/home/wwagner4/work/rust/out";
        let path = format!("{base_path}/cairo-{index:05}.png");
        let mut file = File::create(&path).expect(&format!("Couldn't create '{path}'"));
        surface.write_to_png(&mut file).expect("Could not render image");
        println!("Wrote {path}");
    }

    let n = 20;
    let pattern = read_image();
    let index_range: Vec<i16> = std::ops::Range { start: 0, end: n }.into_iter().collect();
    index_range.par_iter().for_each(|i| draw(&pattern, i, n));
}






