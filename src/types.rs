use quick_xml::{
    Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use std::{collections::HashMap, io::Cursor};

pub type XmlAttrs = HashMap<String, String>;

#[derive(Debug, Clone)]
pub enum XmlChild {
    Element(XmlElement),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct XmlElement {
    pub tag: String,
    pub attributes: XmlAttrs,
    pub children: Vec<XmlChild>,
}

#[derive(Debug, Clone)]
pub struct XmlSegment(pub Vec<XmlChild>);

// Trait conversions to enforce type safety during macro interpolation (@var)
impl From<XmlSegment> for Vec<XmlChild> {
    fn from(seg: XmlSegment) -> Self {
        seg.0
    }
}

impl From<XmlElement> for XmlChild {
    fn from(el: XmlElement) -> Self {
        XmlChild::Element(el)
    }
}

impl From<String> for XmlChild {
    fn from(s: String) -> Self {
        XmlChild::Text(s)
    }
}

impl From<&str> for XmlChild {
    fn from(s: &str) -> Self {
        XmlChild::Text(s.to_string())
    }
}

impl From<Vec<XmlChild>> for XmlSegment {
    fn from(vec: Vec<XmlChild>) -> Self {
        XmlSegment(vec)
    }
}

impl XmlSegment {
    pub fn render(&self) -> anyhow::Result<String> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

        fn write_node(
            writer: &mut Writer<Cursor<Vec<u8>>>,
            child: &XmlChild,
        ) -> anyhow::Result<()> {
            match child {
                XmlChild::Element(node) => {
                    let mut elem = BytesStart::new(&node.tag);
                    for (k, v) in &node.attributes {
                        elem.push_attribute((k.as_str(), v.as_str()));
                    }
                    writer.write_event(Event::Start(elem))?;

                    // Recursively process children
                    for inner_child in &node.children {
                        write_node(writer, inner_child)?;
                    }

                    writer.write_event(Event::End(BytesEnd::new(&node.tag)))?;
                }
                XmlChild::Text(raw_text) => {
                    writer.write_event(Event::Text(BytesText::new(raw_text)))?;
                }
            }
            Ok(())
        }

        for root_node in &self.0 {
            write_node(&mut writer, root_node)?;
        }

        let result = String::from_utf8(writer.into_inner().into_inner())?;
        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n{}",
            result
        ))
    }
}
