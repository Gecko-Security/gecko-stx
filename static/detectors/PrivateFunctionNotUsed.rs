use gecko::Node;
use gecko::visitor::Visitor;

struct PrivateFunctionNotUsed {
    private_fns_names: Vec<Node>,
    msg: String,
    footnote: Option<String>,
    help: Option<String>,
}

impl PrivateFunctionNotUsed {
    fn new() -> Self {
        PrivateFunctionNotUsed {
            private_fns_names: Vec::new(),
            msg: "This private function is not used.".to_string(),
            footnote: Some({FIX}.to_string()),
            help: None,
        }
    }
}

impl Visitor for PrivateFunctionNotUsed {
    fn visit_node(&mut self, node: &Node, run_number: usize) {
        if run_number == 1 && node.kind() == "private_function" {
            self.private_fns_names.push(*node);
            return;
        }

        if run_number == 2 {
            if node.kind() == "contract_function_call" {
                self.private_fns_names.retain(|saved| {
                    saved.child(2).unwrap().child(1).unwrap().utf8_text().unwrap() != node.child(1).unwrap().utf8_text().unwrap()
                });
            }

            if node.kind() == "fold" || node.kind() == "map" || node.kind() == "filter" {
                self.private_fns_names.retain(|saved| {
                    saved.child(2).unwrap().child(1).unwrap().utf8_text().unwrap() != node.parent().unwrap().parent().unwrap().child(2).unwrap().utf8_text().unwrap()
                });
            }
        }

        if run_number == 3 {
            for n in &self.private_fns_names {
                self.add_finding(*n, *n);
            }
            self.private_fns_names.clear();
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
