#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "opml")]
pub struct Opml {
    #[serde(rename = "version", default)]
    pub version: Option<String>,
    pub head: Head,
    pub body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Head {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateCreated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateModified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerEmail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansionState: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertScrollState: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowTop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowLeft: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowBottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowRight: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "outline")]
    pub outlines: Vec<Outline>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Outline {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "text", default)]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "htmlUrl", default)]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language", default)]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title", default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type", default)]
    pub ty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "version", default)]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "xmlUrl", default)]
    pub xml_url: Option<String>,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::Opml;

    #[test]
    fn test_base() {
        for name in std::fs::read_dir("assets").unwrap() {
            println!("{:?}", name);
            let txt = std::fs::read_to_string(name.unwrap().path()).unwrap();
            println!("{:?}", txt);
            let opml: Opml = from_str(&txt).expect("解析 opml 失败");
            println!("{:?}", opml);
        }
    }
}
