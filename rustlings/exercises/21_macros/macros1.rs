macro_rules! my_macro {
    (string: $string:expr) => {
        println!("Check out my macro: {}", $string);
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!(string: "Hello, world!");
}
