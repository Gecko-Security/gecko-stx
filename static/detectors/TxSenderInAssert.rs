use gecko::Node;
use gecko::visitor::{Visitor, NodeIterator};

struct TxSenderInAssert {
    msg: String,
    footnote: Option<String>,
    help: Option<String>,
}

impl TxSenderInAssert {
    fn new() -> Self {
        TxSenderInAssert {
            msg: "Use of tx-sender inside an assert".to_string(),
            footnote: Some({FIX}.to_string()),
            help: None,
        }
    }
}

impl Visitor for TxSenderInAssert {
    fn visit_node(&mut self, node: &Node, i: usize) {
        if i > 1 {
            return;
        }
        if node.utf8_text().unwrap() == "asserts!" {
            for n in NodeIterator::new(node.parent().unwrap()) {
                if n.utf8_text().unwrap() == "tx-sender" && n.kind() == "global" {
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
