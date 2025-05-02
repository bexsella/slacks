// 
// 
// 
// 

// TODO: Organise this properly
mod slacks;
mod tower;
mod geo;
mod maths;

fn main() {
    slacks::pretty_print(slacks::from_str("1234567890abcdefghijklmnopqrstuvwxyz"))
}
