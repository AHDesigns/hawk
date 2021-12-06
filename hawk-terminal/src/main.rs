use hawk::buffers;

fn main() {
  let b = buffers::open_buffer(std::path::Path::new("Cargo.toml")).expect("could not read file");
  println!("Hello, world! {}", b.name);
}
