mod compress; // Declare the compressor module
mod decompress; // Declare the decompressor module

fn main() {
    // Call functions from the modules
    let source = "book.pdf";
    let compressed = "compressed_book.gz";
    let decompressed = "decompress.pdf";

    println!("Compressing...");
    compress::compress(source, compressed);

    println!("Decompressing...");
    decompress::decompress(compressed, decompressed);
}
