use nickel::{Request, Response, MiddlewareResult, QueryString};
use hyper::uri::RequestUri;
use hyper::method::Method;
use hyper::header::{Headers, Host, Referer};

// use common;

pub fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let uri: String = get_uri(&req.origin.uri);
    let method: String = get_method(&req.origin.method);


    println!("LOGGER    {}    {}", method, uri);
    print_headers(&req.origin.headers);
    print_query(req);
    // panic!("");
    res.next_middleware()
}

fn get_uri(uri: &RequestUri) -> String {
    match uri {
        &RequestUri::AbsolutePath(ref result) => { result.to_string() },
        &RequestUri::AbsoluteUri(ref result) => { result.to_string() },
        &RequestUri::Authority(ref result) => { result.to_string() },
        &RequestUri::Star => { "*".to_string() },
    }
}

fn get_method(method: &Method) -> String {
    match method {
        &Method::Extension(ref result) => { result.to_string() },
        _ => { method.to_string() },
    }
}

fn print_headers(headers: &Headers) {
    println!("LOGGER    HEADERS  =  {:?}", headers);
    println!("LOGGER    HOST  =  {:?}", headers.get::<Host>());
    println!("LOGGER    REFERRER  =  {:?}", headers.get::<Referer>());
}

fn print_query(req: &mut Request) {
    let query = req.query();
    println!("LOGGER    QUERY  =  {:?}", query);
}
