# XML Builder Macro

An easy way to build XML in rust directly in a macro.

## Features

- Interpolating built XML
- Expressions for tags, attributes, and values
- Interpolating attribute sets

## Usage

The `xml!` macro builds an XML out of our DSL. An element can be one of:

- name, [ properties ] { body }
- @variable;
- text;

The macro will parse multiple elements back to back and the name and text can be an expression.

The `xml_props!` macro parses properties on an XML element. It is parsed as a sequence of:

- key => value
- @attrset

Where key and value both can be expressions, as long as they can be made into strings.
attrsets (HashMap<String, String>) may be made manually or with the `xml_props!` macro, and supports inline expressions.

Use `.render()` to export it as a string.

## Sample Usage

```rs
let external_attrs = xml_props!("name" => "xml");
let node_name = "node";
let key = "xyz";
let attribute = "zzz";

let external_segment = xml! {
    node_name, [ key => attribute ] {
        "Name", [] { "123"; }
    }
};

let doc = xml! {
    "node2", [ "xmlns" => "123" ] {
        "Leading pure text block. ";
        "settings", [ @external_attrs, "a" => "b", @xml_props!("inline" => attribute) ] {
            @external_segment;
        }
        "Interstitial text fragment. ";
        "component", [ "arch" => "amd64", "status" => "active" ] {
            "Inside element text content";
        }
        "Trailing text block.";
    }
};

let rendered_xml = doc
    .render()
    .expect("Failed to render XML segment data tree");

println!("{}", rendered_xml);
```
