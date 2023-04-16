//Use is similar to 'using' in c++ where it allows calls without the explicit namespace.
use crate::garden::vegetables::Asparagus;
pub mod garden;
mod front_of_house;

//Can use nested paths to stop clutter.
use std::collections::{HashSet, HashMap};

//Can use glob operator for all paths in a scope.
use std::io::*;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    garden::hello();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::winning();
}
