// Importing external crate
extern crate flate2;

// Defining the tools and functions to be used in this project
use flate2::write::GzEncoder; // Function which is used to compress
use flate2::Compression; // actual compresser
use std::env::args; // Read the file name from the user
use std::fs::File; // To read the contents to be compressed
use std::io::copy; // Its required to copy the contents from one package to another
use std::io::BufReader; // Read the compress partail buffer content
use std::time::Instant; // Required when we want to specify the time required for compression

// We use unwrap method to give me the result of the computation & if there is an error stop the program
fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    // Creating a mutable input variable to store the file name
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // Actual Compression is started
    let mut encoder = GzEncoder::new(output, Compression::default());
    let _start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Targe len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed Time: {:?}", _start.elapsed());
}
