// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

/* Note: The help text you get from `rustc` here is a red herring. 
   Follow the suggestion from the rustling's hint instead. */

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

