#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
/// Struct containing response data for an API server HTTP request
pub struct JsonResponse {
    /// Equivalent HTTP Status Code, as server intends to respond with 200(OK)
    pub status_code: i64,
    /// Error flag to notify error has occurred
    pub error: bool,
    /// Optional Message containing API server text response
    pub message: Option<String>,
    /// Optional API Server Access Token, provided on authentication and re-authentication
    pub access_token: Option<String>,
    /// Optional API Server Refresh Token, only provided on initial authentication
    pub refresh_token: Option<String>,
    /// Optional API Server Data response to original request
    pub data: Option<String>,
}

impl JsonResponse {
    /// Method to create a new JsonResponse object
    pub fn new(sc: i64, err: bool, msg: Option<&str>, atok: Option<&str>, rtok: Option<&str>, data: Option<&str>) -> JsonResponse {
        JsonResponse {
            status_code: sc,
            error: err,
            message: match msg {
                Some(ref msg) => Some(msg.to_string()),
                None => None,
            },
            access_token: match atok {
                Some(ref tok) => Some(tok.to_string()),
                None => None,
            },
            refresh_token: match rtok {
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
