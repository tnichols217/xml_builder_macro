#[macro_export]
macro_rules! xml_props {
    () => { $crate::types::XmlAttrs::new() };
    ( $( $tokens:tt )+ ) => {
        {
            let mut map = $crate::types::XmlAttrs::new();
            $crate::xml_props_inner!(map; $( $tokens )*);
            map
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! xml_props_inner {
    ( $map:ident ; $key:expr => $val:expr , $( $tail:tt )* ) => {
        $map.insert($key.to_string(), $val.to_string());
        $crate::xml_props_inner!($map ; $( $tail )*);
    };
    ( $map:ident ; $key:expr => $val:expr ) => {
        $map.insert($key.to_string(), $val.to_string());
    };
    ( $map:ident ; @$var:expr ) => {
        let attrs: $crate::types::XmlAttrs = $var;
        $map.extend(attrs);
    };
    ( $map:ident ; @$var:expr , $( $tail:tt )* ) => {
        let attrs: $crate::types::XmlAttrs = $var;
        $map.extend(attrs);
        $crate::xml_props_inner!($map ; $( $tail )*);
    };
    ( $map:ident ; ) => {};
}

// Global Entry Point
#[macro_export]
macro_rules! xml {
    ( $( $tokens:tt )* ) => {
        {
            let mut nodes = Vec::new();
            $crate::xml_body_inner!(nodes; $( $tokens )*);
            $crate::types::XmlSegment(nodes)
        }
    };
}

// Sequential Mixed-Content Body
#[macro_export]
#[doc(hidden)]
macro_rules! xml_body_inner {
    // Full structural nested element: "tag", [props] { body }
    ( $nodes:ident ; $tag:expr, [ $( $props:tt )* ] { $( $body:tt )* } $( $tail:tt )* ) => {
        let inner_seg = $crate::xml!( $( $body )* );
        $nodes.push($crate::types::XmlChild::Element($crate::types::XmlElement {
            tag: $tag.to_string(),
            attributes: $crate::xml_props!( $( $props )* ),
            children: inner_seg.0,
        }));
        $crate::xml_body_inner!($nodes ; $( $tail )*);
    };

    // External variable: @variable
    ( $nodes:ident ; @$var:expr; $( $tail:tt )* ) => {
        // Enforces type validation via standard Into/From traits
        let extracted: Vec<$crate::types::XmlChild> = Vec::from($var.clone());
        $nodes.extend(extracted);
        $crate::xml_body_inner!($nodes ; $( $tail )*);
    };

    // Raw text: "My String Text Layout"
    ( $nodes:ident ; $text:expr; $( $tail:tt )* ) => {
        $nodes.push($crate::types::XmlChild::Text($text.to_string()));
        $crate::xml_body_inner!($nodes ; $( $tail )*);
    };

    // Base Case: Stream completely consumed
    ( $nodes:ident ; ) => {};
}
