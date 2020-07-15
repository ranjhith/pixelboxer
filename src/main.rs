use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::convert::TryFrom;

fn trim_end_of_line(s: &String) -> &str {
    let len = s.len();
    match &s.chars().nth(len-1).unwrap() {
        '\n' => &s[0..(len-1)],
        _ => &s[0..len],
    }    
}

fn main() {

    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 4);
    
    let in_filename = &args[1];
    let out_filename = &args[2];
    let blowup: usize = args[3].parse().unwrap();

    let path = Path::new(in_filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let in_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(in_file) => in_file,
    };

    let mut reader = BufReader::new(in_file);

    let mut header_p6 = String::new();
    let mut header_width_height = String::new();
    let mut header_height = String::new();
    let mut header_maxval = String::new();
    
    reader.read_line(&mut header_p6);
    assert_eq!(trim_end_of_line(&header_p6), "P6");

    reader.read_line(&mut header_width_height);
    let mut iter = trim_end_of_line(&header_width_height).split_ascii_whitespace();
    let width = iter.next().unwrap().parse::<usize>().unwrap();
    
    let height: usize;
    let next = iter.next();
    if next.is_none() {
        reader.read_line(&mut header_height);
        height = trim_end_of_line(&header_height).parse::<usize>().unwrap();
    } else {
        height = next.unwrap().parse::<usize>().unwrap();
    }   
    
    reader.read_line(&mut header_maxval);
    assert_eq!(trim_end_of_line(&header_maxval).parse::<usize>().unwrap(), 255);

    println!("{}: {} x {} pixels", display, width, height);

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut in_vec_u8: Vec<u8> = Vec::new();
    let in_size = match reader.read_to_end(&mut in_vec_u8) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(in_size) => in_size,
    };

    let path = Path::new(out_filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let out_file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(out_file) => out_file,
    };

    let mut writer = BufWriter::new(&out_file);

    write!(&mut writer, "P6\n{}\n{} {}\n", width * blowup, height * blowup, 255);

    assert_eq!(in_vec_u8.len(), width * height * 3);

    let mut out_vec: Vec<u8> = Vec::new();

    // Set Black while resize
    out_vec.resize(in_size * usize::try_from(blowup * blowup).unwrap(), 0);

    let border = 1;

    for out_index_y in (0..(height * blowup)).step_by(blowup) {
        for out_index_x in (0..(width * blowup)).step_by(blowup) {
            for tempy in border..(blowup - border) {
                for tempx in border..(blowup - border) {
                    let in_index_x = out_index_x / blowup;
                    let in_index_y = out_index_y / blowup;
                    let in_index = in_index_y * width + in_index_x;
                    let out_index = ((out_index_y + tempy) * width * blowup) + out_index_x + tempx;
                    // if in_index == 0 || in_index == 1 || in_index == 1791 {
                    //     println!("in_index: {} out_index: {} out_index_x: {} out_index_y: {} tempx: {} tempy: {}", in_index, out_index, out_index_x, out_index_y, tempx, tempy);
                    // }
                    out_vec[out_index * 3] = in_vec_u8[in_index * 3];
                    out_vec[out_index * 3 + 1] = in_vec_u8[in_index * 3 + 1];
                    out_vec[out_index * 3 + 2] = in_vec_u8[in_index * 3 + 2];
                }
            }
        }
    }

    writer.write(&out_vec[..]);

    drop(writer);
}