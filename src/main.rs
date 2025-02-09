use serde_opml::Opml;
use serde_xml_rs::from_str;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let txt = std::fs::read_to_string(path).unwrap();
        let opml: Opml = from_str(&txt).unwrap();
        let json = serde_json::to_string(&opml).unwrap();
        println!("{}", json);
    }else{
      println!("serde-opml <file.opml>")
    }
}
