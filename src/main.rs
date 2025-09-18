// src/main.rs — either delete it, or replace with a minimal demo

use uvoxid::UvoxId;

fn main() {
    let id = UvoxId::new(0, 6_371_000_000_000, 0, 0);
    println!("Earth center: {:?}", id);
}
