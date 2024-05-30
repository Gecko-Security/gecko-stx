<h1 align="center" style="font-size: 200px;">
  <strong>GECKO</strong>
</h1>
<p align="center">
    <br />
        <img src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/273f6ecc-8cb1-4d73-88d1-320513053c69" width="225" alt=""/></a>
    <br />
</p>

<p align="center"><strong>The first Clarity static analyser that finds bugs in your smart contracts
</strong></p>

### What is Gecko?


### Images

Demo: 

### How Gecko's Static Analysis Works
1. We parse the Clarity code into a structure that Gecko can understand, this is called an Abstract Syntax Tree (AST). It represents the hirearchical structure of the code. We use the [Clarity Contract Analysis Crate](https://docs.rs/stacks-codec/latest/stacks_codec/clarity/vm/analysis/types/struct.ContractAnalysis.html), which converts Clarity code into an AST and other metadata. This is the main entrypoint for Gecko.
2. We then define a struct called [Gecko](), which implements the [`ast_visitor`](https://doc.rust-lang.org/stable/nightly-rustc/rustc_ast/visit/trait.Visitor.html) crate used to traverse each node and understand the behavior of the code.
3. Taint analysis is used to track the flow of potentially unsafe data through the program and locate bugs and vulnearbilities. This imvolves defining the vulnearbility detectors as invariants and tracking the data to ensure it is properly checked or sanitized.
4. As Gecko traverses the tree it propagates this taint to other nodes that depend on these.
5. Once the traversal is complete messages are displayed about issues found including the location of the bug in source. 

<p align="center">
    <br />
        <img src="https://github.com/Gecko-Security/Gecko-Clarity/assets/22000925/b651810b-6db5-457c-88d1-7ee5feb6dc01" width="800" alt=""/></a>
    <br />
</p>

_Example of a traversal of AST_
