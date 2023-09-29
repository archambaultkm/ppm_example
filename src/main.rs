use std::fs::File;
use std::io::Write;

const IMAGE_HEIGHT: u32 = 100;
const IMAGE_WIDTH: u32 = 256;
const MAX_VALUE: u8 = 255;

fn main() {
    //create a file
    let mut data_file = File::create("ppm_example.ppm").expect("Creation failed.");
    let cols = 200;
    let rows = 100;

    data_file.write("P3\n".as_bytes()).expect("write failed");
    data_file.write((cols.to_string() + " " + &*rows.to_string() + "\n").as_bytes()).expect("write failed");
    data_file.write("255\n".as_bytes()).expect("write failed");

    for j in (0..rows-1).rev() {
        for i in 0..cols {
            let r = i as f64/ cols as f64;
            let g = j as f64/ rows as f64;
            let b = 0.2;

            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;

            let ir = ir.to_string();
            let ig = ig.to_string();
            let ib = ib.to_string();

            data_file.write((ir.clone() + " " + &*ig + " " + &*ib + "\n").as_bytes()).expect("write failed");
            //println!("{} {} {}", ir, ig, ib);
        }
    }

    println!("Created a file");
}
