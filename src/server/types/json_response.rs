#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub struct JsonResponse {
    pub status_code: i64,
    pub error: bool,
    pub message: Option<String>,
    pub token: Option<String>,
    pub data: Option<String>,
}

impl JsonResponse {
    pub fn new(sc: i64, err: bool, msg: Option<&str>, tok: Option<&str>, data: Option<&str>) -> JsonResponse {
        JsonResponse {
            status_code: sc,
            error: err,
            message: match msg {
                Some(ref msg) => Some(msg.to_string()),
                None => None,
            },
            token: match tok {
                Some(ref tok) => Some(tok.to_string()),
                None => None,
            },
            data: match data {
                Some(ref data) => Some(data.to_string()),
                None => None,
            },
        }
    }
}
