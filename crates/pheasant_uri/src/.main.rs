use pheasant_uri::{lex, semantic_tree, syntax_tree};

fn main() {
    let uri = "https://developer.mozilla.org/en-US/docs/Skills/Infrastructure/Understanding_URLs";
    let uri = "https://developer.mozilla.org/en-US/search?q=URL";
    let uri = "//developer.mozilla.org/en-US/docs/Learn_web_development";
    let uri = "/en-US/docs/Learn_web_development";
    let uri = "http://example.com/kb/index.php?cat=1&id=23";
    // FIXME this generates a trailing empty Seq("")
    let uri = "https://username:password@www.example.com:80/";
    // let uri = "http://localhost:3422";
    let uri = "http://a/%%30%30";
    println!("{}", uri);

    let lex = lex(uri);
    println!("{:?}", lex);

    let syn_tree = syntax_tree(lex).unwrap();
    println!("{:?}", syn_tree);

    let sem_tree = semantic_tree(syn_tree);
    println!("{:?}", sem_tree);
}
