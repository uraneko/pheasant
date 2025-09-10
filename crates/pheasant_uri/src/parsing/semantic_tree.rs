//! semantic analysis
//! parses the components into their values
//! also validates the tokens contents

use hashbrown::HashSet;

enum Component {
    Scheme,
    Auth,
    Domain,
    Port,
    Path,
    Query,
    Fragment,
}

/// every component is parsed into its value
struct AnnotatedComponentTree {
    components: HashSet<Component>,
}
