use gecko::Node;
use gecko::visitor::{Visitor, NodeIterator};

struct CallInsideAsContract {
    call: bool,
    lit: bool,
    checked: Vec<Node>,
    msg: String,
    help: Option<String>,
    footnote: Option<String>,
}

impl CallInsideAsContract {
    fn new() -> Self {
        CallInsideAsContract {
            call: false,
            lit: false,
            checked: Vec::new(),
            msg: {FIX}.to_string(),
            help: None,
            footnote: None,
        }
    }
}

impl Visitor for CallInsideAsContract {
    fn visit_node(&mut self, node: &Node, i: usize) {
        if i > 1 {
            return;
        }
        if node.utf8_text().unwrap() == "as-contract" {
            for n in NodeIterator::new(node.parent().unwrap()) {
                if n.utf8_text().unwrap() == "contract-call?" {
                    self.call = true;
                }
                if n.grammar_name().unwrap() == "contract_principal_lit" {
                    self.lit = true;
                }
            }

            if self.call && !self.lit && !self.checked.contains(node) {
                self.add_finding(node.parent().unwrap(), node);
                self.checked.push(*node);
            }
        }

        self.call = false;
        self.lit = false;
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
