// 
// 
// 
// 

// TODO: Organise this properly
mod slacks;
mod tower;
mod geo;
mod maths;
mod config;

fn main() {
    slacks::Semaphore::pretty_print(
        slacks::Semaphore::from_str("1234567890abcdefghijklmnopqrstuvwxyz")
    )
}
