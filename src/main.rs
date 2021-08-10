use pom::char_class::alphanum;
use pom::parser::*;
use std::convert::identity;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let ts_types = include_bytes!("../node_modules/react-feather/dist/index.d.ts");
    let types = Icon::parse(ts_types);
    let mut file = File::create("src/ReactFeather.res")?;

    let file_content = [
        Icon::print_components(&types),
        Icon::print_type(&types),
        Icon::print_patterns(&types),
    ]
    .join("\n\n");

    file.write_all(file_content.as_bytes())?;

    Ok(())
}

#[derive(Debug)]
struct Icon(String);

impl Icon {
    fn print_component(&self) -> String {
        let Icon(name) = self;
        format!(
            "
            module {0} = {{
                @module(\"react-feather\")
                @react.component
                external make: (~color: string=?, ~size: int=?, ~className: string=?) => React.element = \"{0}\"
            }}",
            name
        )
    }

    pub fn print_components(icons: &Vec<Icon>) -> String {
        icons
            .into_iter()
            .map(|icon| icon.print_component())
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    pub fn print_type(icons: &Vec<Icon>) -> String {
        let beginning = "type name = [";

        let variants = icons
            .into_iter()
            .map(|Icon(name)| format!("|#\"{}\"", name))
            .collect::<Vec<String>>()
            .join("\n");

        let ending = "]";

        [beginning, &variants, ending].join("\n")
    }

    pub fn print_patterns(icons: &Vec<Icon>) -> String {
        let beginning = "
        @react.component
        let make = (~name: name, ~color=?, ~size=?, ~className=?) => {
            switch name {";

        let matches = icons
            .into_iter()
            .map(|Icon(name)| format!("| #\"{0}\" => <{0} ?color ?size ?className />", name))
            .collect::<Vec<String>>()
            .join("\n");

        let ending = "}}";

        [beginning, &matches, ending].join("\n")
    }

    pub fn parse<'a>(icons: &[u8]) -> Vec<Icon> {
        let name_parser =
            ((seq(b"export const ") * is_a(alphanum).repeat(1..) - seq(b": Icon;")).map(|n| {
                let name = String::from_utf8(n).unwrap();
                Some(Icon(name))
            })) | is_a(|x| x != b'\n').repeat(0..).map(|_| None);

        let parser = list(name_parser, sym(b'\n'));

        parser
            .parse(icons)
            .unwrap()
            .into_iter()
            .filter_map(identity)
            .collect()
    }
}
