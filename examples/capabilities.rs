use lib::grammar::{
    ast::{
        literal::LiteralTypes,
        node::{tree, Node::ControlRequire},
    },
    parser::parse,
};

fn main() {
    let sieve_script = br#"
    require ["fileinto", "body", "imap4flags"];
    "#;

    let contents = String::from_utf8(sieve_script.to_vec()).unwrap();
    let pairs = parse(&contents).unwrap_or_else(|e| panic!("{}", e));
    let nodes = tree(pairs.clone(), vec![]);

    let capabilities = nodes.into_iter().map(|n| match *n {
        ControlRequire(n) => n.capabilities,
        _ => Some(LiteralTypes::Array(vec![])),
    });

    println!("{:#?}", capabilities);

    // for node in nodes {
    //     match *node {
    //         ControlRequire(ref node) => {
    //             println!("{:#?}", node.capabilities);
    //         }
    //         _ => {}
    //     }
    // }
}
