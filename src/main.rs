use std::env;

use rb_xml::Analyzer;
use rb_xml::RekordboxXml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        println!("Please provide the path to a Rekordbox XML file.");
        println!("Additionally, you can also provide the path to a directory containing tracks.");
        return Ok(());
    }

    let parser = RekordboxXml::from_file(&args[1])?;
    parser.write_to_file("out.xml")?;

    if args.len() == 3 {
        let mut analyzer = Analyzer::new(parser, &args[2]);
        analyzer.run();
        dbg!("{:?}", analyzer.report);
    }

    Ok(())
}
