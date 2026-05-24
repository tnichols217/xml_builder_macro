mod macros;
mod types;

fn main() {
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
}
