use gecko::Node;
use gecko::visitor::{Visitor, NodeIterator};

struct DivideBeforeMultiply {
    msg: String,
    footnote: Option<String>,
    help: Option<String>,
}

impl DivideBeforeMultiply {
    fn new() -> Self {
        DivideBeforeMultiply {
            msg: "Precision loss of divide inside a multiplication.".to_string(),
            footnote: Some({FIX}.to_string()),
            help: None,
        }
    }
}

impl Visitor for DivideBeforeMultiply {
    fn visit_node(&mut self, node: &Node, i: usize) {
        if i > 1 {
            return;
        }
        if node.kind() == "native_identifier" && node.utf8_text().unwrap() == "*" {
            for n in NodeIterator::new(node.parent().unwrap()) {
                if n.utf8_text().unwrap() == "/" && n.kind() == "native_identifier" {
                    self.add_finding(node.parent().unwrap(), node);
                }
            }
        }
    }

    fn msg(&self) -> &str {
        &self.msg
    }

    fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    fn footnote(&self) -> Option<&str> {
        self.footnote.as_deref()
    }
}
