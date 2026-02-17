use std::io::Write;

fn main() {
    let filename = std::env::args().skip(1).last();

    if filename.is_none() {
        eprintln!("No arguments!");
        std::process::exit(1);
    }

    let data = std::fs::read(filename.unwrap()).unwrap();

    let mut image = pixal::tga::from_tga_data(data.as_slice()).unwrap();
    image.rotate_left();

    println!("use `ffplay -f rawvideo -video_size {}x{} -pixel_format rgb24 -i out.bin`", image.width(), image.height());
    
    {
        let mut file = std::fs::File::create("out.bin").unwrap();

        file.write(image.data()).unwrap();
    }

    println!("ok");
}
