use gecko::Node;
use gecko::visitor::Visitor;

struct UnwrapPanicUsage {
    msg: String,
    footnote: Option<String>,
    help: Option<String>,
}

impl UnwrapPanicUsage {
    fn new() -> Self {
        UnwrapPanicUsage {
            msg: "Improper unwrap-panic.".to_string(),
            footnote: Some({FIX}.to_string()),
            help: None,
        }
    }
}

impl Visitor for UnwrapPanicUsage {
    fn visit_node(&mut self, node: &Node, i: usize) {
        if i > 1 {
            return;
        }
        if node.kind() == "unwrap-panic" {
            self.add_finding(node.parent().unwrap(), node);
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
