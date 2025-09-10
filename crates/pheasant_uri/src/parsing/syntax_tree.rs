// syntax analysis
// builds a components tree where each component contains it's tokens and their meta
// validates that the order of components makes sense
// checks that the component combination is allowed

enum Component {
    Nid,
    Nss,
    Scheme,
    Auth,
    Domain,
    Port,
    Path,
    Query,
    Fragment,
}

pub struct ComponentTree {
    components: Vec<Component>,
}
