use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}

struct Icon(String);

impl Icon {
    pub fn print(&self) -> String {
        let Icon(name) = self;
        format!(
            "
module {0} = {{
    @module(\"react-feather\")
    @react.component
    external make: props = \"{0}\"
}}",
            name
        )
    }

    pub fn print_pattern(icons: Vec<Icon>) -> String {
        let beginning = "
@react.component
let make = (~name, ~color, ~size) => {{
    switch name {{\n";
    } 
}
