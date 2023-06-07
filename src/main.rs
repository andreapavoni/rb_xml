use rb_xml::RekordboxXml;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide the path to a Rekordbox XML file.");
        return;
    }

    if let Ok(parser) = RekordboxXml::from_file(&args[1]) {
        parser.write_to_file("out.xml");
    } else {
        println!("Failed to read the XML file.");
    }
}
