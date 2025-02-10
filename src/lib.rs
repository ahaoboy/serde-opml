use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

fn deserialize_vec_u32<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let numbers = s
        .split(',')
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<u32>().map_err(de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(numbers)
}

fn serialize_vec_u32_as_comma<S>(value: &[u32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = value
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",");
    serializer.serialize_str(&joined)
}

fn deserialize_vec_str<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let numbers = s.split(',').map(|i| i.to_string()).collect::<Vec<_>>();
    Ok(numbers)
}

fn serialize_vec_str_as_comma<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = value.join(",");
    serializer.serialize_str(&joined)
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "opml")]
pub struct Opml {
    #[serde(rename = "@version")]
    pub version: String,
    pub head: Head,
    pub body: Body,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "head")]
pub struct Head {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "ownerName", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "ownerEmail", skip_serializing_if = "Option::is_none")]
    pub owner_email: Option<String>,

    #[serde(
        rename = "expansionState",
        deserialize_with = "deserialize_vec_u32",
        serialize_with = "serialize_vec_u32_as_comma",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub expansion_state: Vec<u32>,

    #[serde(rename = "vertScrollState", skip_serializing_if = "Option::is_none")]
    pub vert_scroll_state: Option<i32>,
    #[serde(rename = "windowTop", skip_serializing_if = "Option::is_none")]
    pub window_top: Option<i32>,
    #[serde(rename = "windowLeft", skip_serializing_if = "Option::is_none")]
    pub window_left: Option<i32>,
    #[serde(rename = "windowBottom", skip_serializing_if = "Option::is_none")]
    pub window_bottom: Option<i32>,
    #[serde(rename = "windowRight", skip_serializing_if = "Option::is_none")]
    pub window_right: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "body")]
pub struct Body {
    #[serde(rename = "outline")]
    outlines: Vec<Outline>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "outline")]
pub struct Outline {
    #[serde(rename = "@text")]
    pub text: String,

    #[serde(
        rename = "@category",
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_vec_str",
        serialize_with = "serialize_vec_str_as_comma",
        default
    )]
    pub category: Vec<String>,

    #[serde(rename = "@created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(rename = "@isComment", skip_serializing_if = "Option::is_none")]
    pub is_comment: Option<bool>,

    #[serde(rename = "@isBreakpoint", skip_serializing_if = "Option::is_none")]
    pub is_breakpoint: Option<bool>,

    #[serde(rename = "@description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "@htmlUrl", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "@language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<String>,

    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "@xmlUrl", skip_serializing_if = "Option::is_none")]
    pub xml_url: Option<String>,

    #[serde(rename = "@url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "outline", skip_serializing_if = "Vec::is_empty", default)]
    outlines: Vec<Outline>,
}

#[cfg(test)]
mod test {

    use quick_xml::{de::from_str, se::to_string};

    use crate::{Head, Opml, Outline};

    #[test]
    fn test_outline() {
        let s = r#"<outline text="The Mets are the best team in baseball." category="/Philosophy/Baseball/Mets,/Tourism/New York" created="Mon, 31 Oct 2005 18:21:33 GMT"/>"#;
        let outline: Outline = from_str(s).unwrap();
        assert!(outline.text == "The Mets are the best team in baseball.");
        println!("{:?}", outline.category);

        assert!(outline.category.len() == 2);
        assert!(outline.created.unwrap() == "Mon, 31 Oct 2005 18:21:33 GMT");

        let s = r#"	<outline text="x" type="link" url="http://hosting.opml.org/dave/mySites.opml" isComment="true" isBreakpoint="true"
           htmlUrl="http://www.infoworld.com/news/index.html" language="unknown"
      title="x" version="RSS2"
      xmlUrl="http://www.infoworld.com/rss/news.xml"
      >
			<outline text="x" isBreakpoint="true"/>
			<outline text="x"/>
			</outline>"#;
        let outline: Outline = from_str(s).unwrap();
        assert_eq!(
            outline.url.unwrap(),
            "http://hosting.opml.org/dave/mySites.opml"
        );
        assert_eq!(outline.title.unwrap(), "x");
        assert_eq!(outline.ty.unwrap(), "link");
        assert_eq!(outline.version.unwrap(), "RSS2");
        assert!(outline.is_comment.unwrap());
        assert!(outline.is_breakpoint.unwrap());
        assert!(outline.outlines[0].is_breakpoint.unwrap());
        assert_eq!(outline.outlines[1].text, "x");
    }
    #[test]
    fn test_head() {
        let s = r#"<head>
    <title>states.opml</title>
    <dateCreated>Tue, 15 Mar 2005 16:35:45 GMT</dateCreated>
    <dateModified>Thu, 14 Jul 2005 23:41:05 GMT</dateModified>
    <ownerName>Dave Winer</ownerName>
    <ownerEmail>dave@scripting.com</ownerEmail>
    <expansionState>1, 6, 13, 16,18,20</expansionState>
    <vertScrollState>1</vertScrollState>
    <windowTop>106</windowTop>
    <windowLeft>106</windowLeft>
    <windowBottom>558</windowBottom>
    <windowRight>479</windowRight>
  </head>"#;
        let head: Head = from_str(s).unwrap();
        assert_eq!(head.title.unwrap(), "states.opml");
        assert_eq!(head.date_created.unwrap(), "Tue, 15 Mar 2005 16:35:45 GMT");
        assert_eq!(head.date_modified.unwrap(), "Thu, 14 Jul 2005 23:41:05 GMT");
        assert_eq!(head.owner_name.unwrap(), "Dave Winer");
        assert_eq!(head.owner_email.unwrap(), "dave@scripting.com");
        assert_eq!(head.expansion_state.len(), 6);
        assert_eq!(head.vert_scroll_state.unwrap(), 1);
        assert_eq!(head.window_top.unwrap(), 106);
        assert_eq!(head.window_left.unwrap(), 106);
        assert_eq!(head.window_bottom.unwrap(), 558);
        assert_eq!(head.window_right.unwrap(), 479);
    }

    #[test]
    fn test_assets() {
        for name in std::fs::read_dir("assets").unwrap() {
            let txt = std::fs::read_to_string(name.unwrap().path()).unwrap();
            let opml: Opml = from_str(&txt).unwrap();
            assert!(opml.version == "2.0");
            let xml = to_string(&opml).unwrap();
            let opml2: Opml = from_str(&xml).unwrap();
            assert_eq!(opml, opml2);
        }
    }
}
