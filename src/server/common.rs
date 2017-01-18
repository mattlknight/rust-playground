use hyper::status::{StatusCode};
use nickel::{Request, Response, MediaType, MiddlewareResult};
use rustc_serialize::json::{self, DecoderError, ParserError};
use std::io::{self};
use std::io::Read;

use ::types::{LoginUser, JsonResponse};
use ::errors::{ServerError};


pub fn get_body(req: &mut Request) -> Result<String, io::Error> {
    let mut body = String::with_capacity(100);
    try!(req.origin.read_to_string(&mut body));
    Ok(body)
}

pub fn get_user<'a>(req: &'a mut Request) -> Result<LoginUser<'a>, ServerError>{
    let body = try!(get_body(req));
    let user = try!(json::decode::<LoginUser>(&body));
    // let user = LoginUser {
    //     // username: "Robert'); DROP TABLE Students;--?",
    //     username: Cow::from("Bobby Tables"),
    //     password: Cow::from("Secret Squirrel"),
    // };

    Ok(user)
}

pub fn send_json_error<'mw>(mut res: Response<'mw>, err: ServerError) ->
    MiddlewareResult<'mw> {

    let reply = match err {
        ServerError::DecoderError(DecoderError::MissingFieldError(err)) => {
            JsonResponse::new(400, true, Some(&format!("Missing Field [{}]", err)), None, None)
        },
        ServerError::DecoderError(DecoderError::ParseError(err)) => match err {
            ParserError::SyntaxError(msg, ..) => {
                JsonResponse::new(400, true, Some(&format!("Syntax Error [{}]", json::error_str(msg))), None, None)
            },
            _ => {
                println!("UNHANDLED ERROR  {:?}", err);
                JsonResponse::new(500, true, Some("Unhandled Server Error"), None, None)
            },
        },
        ServerError::SqlError(err) => {
            JsonResponse::new(400, true, Some(&format!("{}", err)), None, None)
        },
        _ => {
            println!("UNHANDLED ERROR  {:?}", err);
            JsonResponse::new(500, true, Some("Unhandled Server Error"), None, None)
        },
    };
    let encoded = json::encode(&reply).expect("Error sending json reply");

    res.set(StatusCode::Ok);
    res.set(MediaType::Json);

    println!("REPLYING");
    return res.send(encoded);
}

pub fn json_reply(msg: Option<&str>, data: Option<&str>) -> String {
    let reply = JsonResponse::new(200, false, msg, None, data);
    json::encode(&reply).expect("Error sending json reply")
}

// fn send_json_reply<'mw, D>(mut res: Response<'mw, D>, reply: String) ->
//     MiddlewareResult<'mw, D> {
//
//     res.set(StatusCode::Ok);
//     res.set(MediaType::Json);
//     return res.send(reply);
// }
