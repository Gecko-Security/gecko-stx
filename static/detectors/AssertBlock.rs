use gecko::Node;
use gecko::visitor::{Visitor, NodeIterator};

struct AssertBlockHeight {
    msg: String,
    help: Option<String>,
    footnote: Option<String>,
}

impl AssertBlockHeight {
    fn new() -> Self {
        AssertBlockHeight {
            msg: {FIX}.to_string(),
            help: None,
            footnote: Some("NOTE".to_string()),
        }
    }
}

impl Visitor for AssertBlockHeight {
    fn visit_node(&mut self, node: &Node, i: usize) {
        if i > 1 {
            return;
        }
        if node.utf8_text().unwrap() == "asserts!" {
            for n in NodeIterator::new(node.parent().unwrap()) {
                if n.utf8_text().unwrap() == "block-height" && n.grammar_name().unwrap() == "global" {
                    self.add_finding(node.parent().unwrap(), node);
                    break;
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
