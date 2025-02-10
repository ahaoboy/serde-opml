use quick_xml::de::from_str;
use serde_opml::Opml;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let txt = std::fs::read_to_string(path).unwrap();
        let opml: Opml = from_str(&txt).unwrap();
        println!("{:#?}", opml);
    } else {
        println!("serde-opml <file.opml>")
    }
}
