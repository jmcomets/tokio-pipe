use tokio::prelude::*;
use clap::{App, Arg};

#[allow(unused)]
fn copy_reading_entire_file_in_memory() {
    let input_file = tokio::fs::File::open("in.txt");
    let output_file = tokio::fs::File::create("out.txt");

    let read = input_file
        .and_then(|file| tokio::io::read_to_end(file, vec![]))
        .map(|(_, contents)| {
            println!("read {} bytes", contents.len());
            contents
        })
        .map_err(|err| {
            eprintln!("read error: {:?}", err);
            err
        });

    let copy = output_file.join(read)
        .and_then(|(file, contents)| {
            tokio::io::write_all(file, contents)
        })
        .map(|(_, contents)| {
            println!("wrote {:?} bytes", contents.len());
        })
        .map_err(|err| {
            eprintln!("write error: {:?}", err);
        });

    tokio::run(copy);
}

#[allow(unused)]
fn copy_with_buffering() {
    let input_file = tokio::fs::File::open("in.txt");
    let output_file = tokio::fs::File::create("out.txt");

    let copy = input_file.join(output_file)
        .and_then(|(input_file, output_file)| tokio::io::copy(input_file, output_file))
        .map(|(nb_bytes, _, _)| {
            println!("copied {:?} bytes", nb_bytes);
        })
        .map_err(|err| {
            eprintln!("copy error: {:?}", err);
        });

    tokio::run(copy);
}

fn app() -> App<'static, 'static> {
    App::new("bench-tokio-pipe")
        .version("1.0")
        .author("Jean-Marie Comets <jean.marie.comets@gmail.com>")
        .about("Compare Tokio implementations of a disk read/write pipeline")
        .arg(Arg::with_name("test")
             .help("Test to run (either \"basic\" or \"buffered\")")
             .required(true)
             .index(1)
             )
}

fn main() {
    let matches = app().get_matches();

    match matches.value_of("test").unwrap() {
        "basic"    => copy_reading_entire_file_in_memory(),
        "buffered" => copy_with_buffering(),
        _          => unimplemented!(),
    }
}
