use crate::not_found;
use std::collections::HashMap;

pub fn lookup<S>(req: &mut String, funs: &HashMap<&'static str, S>)
where
    S: crate::Service,
{
    if let Some(service) = funs
        .iter()
        .find_map(|(condition, service)| req.contains(condition).then(|| service))
    {
        service.run(req)
    } else {
        not_found(req)
    }
    // // println!("\n{}", req);
    // if &req[3..17] == " /favicon.ico " || &req[7..21] == " /favicon.ico " {
    //     favicon(req);
    // } else if &req[..6] == "GET / " || &req[..19] == "GET /?bg=dark-grey " {
    //     index(req);
    // } else {
    //     not_found(req);
    // }
}
