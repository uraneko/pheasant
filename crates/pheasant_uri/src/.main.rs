use pheasant_uri::{Parse, PathRelativeUrl, Url, Urn};

fn main() {
    let uri = "https://developer.mozilla.org/en-US/docs/Skills/Infrastructure/Understanding_URLs";
    // let uri = "https://developer.mozilla.org/en-US/search?q=URL";
    // let uri = "//developer.mozilla.org/en-US/docs/Learn_web_development";
    // let uri = "/en-US/docs/Learn_web_development";
    let uri = "http://example.com/kb/index.php?cat=1&id=23";
    // let uri = "https://username:password@www.example.com:80/";
    // let uri = "http://localhost:3422?tera=543&hy#fewlm3;kr 3,4f";
    // let uri = "http://a/%%30%30";
    let urn = "URN:example:a123,z456";

    // println!("{:?}", urn.parse::<Urn>());

    println!("{}\n", uri);

    let lex = Url::lex(uri);
    println!("lex -> {:?}\n", lex);

    let syn_tree = Url::syntax_tree(lex.unwrap());
    println!("syn -> {:?}\n", syn_tree);

    let sem_tree = Url::semantic_tree(syn_tree.unwrap());
    println!("sem -> {:?}\n", sem_tree);

    println!("{:#?}", uri.parse::<Url>());
}
