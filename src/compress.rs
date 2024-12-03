// Compression logic
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, copy};
use std::time::Instant;

pub fn compress(source: &str, target: &str) {
    let start = Instant::now();

    let mut input = BufReader::new(File::open(source).expect("Failed to open source file"));
    let output = File::create(target).expect("Failed to create target file");
    let mut encoder = GzEncoder::new(output, Compression::default());

    match copy(&mut input, &mut encoder) {
        Ok(bytes) => println!("Compressed {} bytes in {:?}", bytes, start.elapsed()),
        Err(e) => eprintln!("Failed to compress data: {}", e),
    }

    let output = encoder.finish().expect("Failed to finish compression");
    println!("Source size: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Compressed size: {:?}", output.metadata().unwrap().len());
}



// Compression code if you want to use CLI 
/* 
fn main(){
    /* Input -> reads from the cli, opens the file using BufReader and checks for the 1st argument(nth(1)) */
    let mut input = BufReader:: new(File::open(args().nth(1).unwrap()).unwrap()); 

    /* Output -> creates a file based on the nth(2) argument to store the compressed pdf */
    let output = File::create(args().nth(2).unwrap()).unwrap();

    /* Encoder -> Provides the GzEncoder struct, which wraps an output stream and compresses data written to it in the Gzip format. */
    // Initialise the encoder
    let mut encoder: GzEncoder<File> = GzEncoder::new(output, Compression::default());   // new object for default compressing output

    // Measuring time and compressing the data
    let start = Instant::now();
    // Reads data from the buffered input and writes the compressed data to encoder
    match copy(&mut input,&mut encoder){
        Ok(bytes) => {println!("Copied {} bytes in {:?}",bytes,start.elapsed());}
        Err(e) => {println!("Failed to copy data: {}",e);}
    }
    let output = encoder.finish().expect("Failed to finish compression.");
    println!("Source len: {:?}",input.get_ref().metadata().unwrap().len());
    println!("Target len {:?}",output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
*/