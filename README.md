# Gecko-Clarity
The first static analyser to detect vulnerabilities in clarity code 


### How Gecko's Static Analysis Works
1. We parse the Clarity code into a structure that Gecko can understand, this is called an Abstract Syntax Tree (AST). It represents the hirearchical structure of the code. We use the [Clarity Contract Analysis Crate](https://docs.rs/stacks-codec/latest/stacks_codec/clarity/vm/analysis/types/struct.ContractAnalysis.html), which converts Clarity code into an AST and other metadata. This is the main entrypoint for Gecko.
2. We then define a struct called [Gecko](), which implements the [`ast_visitor`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_ast/visit/trait.Visitor.html) crate used to traverse each node and understand the behavior of the code.
3. Taint analysis is used to track the flow of potentially unsafe data through the program and locate bugs and vulnearbilities. This imvolves defining the vulnearbility detectors as invariants and tracking the data to ensure it is properly checked or sanitized.
4. As Gecko traverses the tree it propagates this taint to other nodes that depend on these.
5. Once the traversal is complete messages are displayed about issues found including the location of the bug in source. 


![AST](https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/b651810b-6db5-457c-88d1-7ee5feb6dc01)
_Example of a traversal of AST_
