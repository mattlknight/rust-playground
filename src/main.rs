// MIT License
//
// Copyright (c) 2017 Matthew Knight
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate hyper;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate plugin;
extern crate mylib;

use hyper::net::{Openssl};
use hyper::status::{StatusCode};
// use plugin::Pluggable;
use nickel::{Nickel, HttpRouter, Router, Request, Response, MediaType, MiddlewareResult};
// use rustc_serialize::json::{DecoderError};
use rustc_serialize::json::{self, DecoderError, ParserError};
// use std::any::Any;
// use std::borrow::Cow;
// use std::error::Error;
use std::net::{SocketAddrV4, Ipv4Addr};
// use std::io::{self, ErrorKind};
use std::io::{self};
use std::io::Read;

use mylib::traits::{SqlSafe};
use mylib::types::{LoginUser, JsonResponse};
use mylib::errors::{ServerError};
// use mylib::errors::{ServerError, LoginError};
use mylib::req_logger;

lazy_static! {
    static ref IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    static ref PORT: u16 = 8080;
    static ref ADDRESS: SocketAddrV4 = SocketAddrV4::new(*IP, *PORT);
}

fn get_body(req: &mut Request) -> Result<String, io::Error> {
    let mut body = String::with_capacity(100);
    try!(req.origin.read_to_string(&mut body));
    Ok(body)
}

fn get_user<'a>(req: &'a mut Request) -> Result<LoginUser<'a>, ServerError>{
    let body = try!(get_body(req));
    let user = try!(json::decode::<LoginUser>(&body));
    // let user = LoginUser {
    //     // username: "Robert'); DROP TABLE Students;--?",
    //     username: Cow::from("Bobby Tables"),
    //     password: Cow::from("Secret Squirrel"),
    // };

    Ok(user)
}

fn send_json_error<'mw>(mut res: Response<'mw>, err: ServerError) ->
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

fn json_reply(msg: Option<&str>, data: Option<&str>) -> String {
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

fn main() {
    let mut server = Nickel::new();
    let mut router: Router = Nickel::router();

    server.utilize(req_logger::logger_fn);

    router.post("/api/authenticate", middleware! { |request, mut response|
        println!("POST /api/authenticate");

        let user: LoginUser = match get_user(request) {
            Ok(user) => {
                user
            },
            Err(err) => {
                return send_json_error(response, err);
            },
        };
        println!("USER =  {}", user);

        match user.is_sql_safe() {
            Ok(user) => {
                println!("User \"{}\" is safe", user.username);
            },
            Err(err) => {
                return send_json_error(response, From::from(err));
            },
        };
        let encoded: String = json_reply(Some(&format!("Welcome {}", user.username)), None);
        response.set(StatusCode::Ok);
        response.set(MediaType::Json);
        (encoded)
    });

    server.utilize(router);

    let ssl = match Openssl::with_cert_and_key("keys/server.crt", "keys/server.key") {
        Ok(ssl) => ssl,
        Err(err) => panic!("Failed to open SSL keys from target/*/keys/; {:?}", err),
    };
    let listening = match server.listen_https(*ADDRESS, ssl) {
        Ok(srv) => srv,
        Err(err) => panic!("Failed to start Nickel Server; {:?}", err),
    };

    println!("Listening on: {:?}", listening.socket());
}



        // let db = request.pg_conn().expect("Failed to get a connection from pool");
        // let db_user = &login_user.username.map(|x| get_login_from_db(&db, &x));


        // if &login_user.password.map(|x| auth::validate_password(&x, &db_user.password_hash)) {
        //     let user_details = get_user_from_db(&db, &db_user.id);
        //
        //     if user_details.email_verified != Some(true) {
        //         let return_status = to_json_status(false, "Pending email verification!");
        //
        //         response.set(StatusCode::NotAcceptable);
        //         response.set(MediaType::Json);
        //         println!("{}  ROUTE   Sending Email Not Verified For ({})", common::local_timestamp(), &login_user.username);
        //         json::encode(&return_status).expect("Failed to serialize response")
        //     } else if user_details.admin_approved != Some(true) {
        //         let return_status = to_json_status(false, "Pending admin approval!");
        //
        //         response.set(StatusCode::NotAcceptable);
        //         response.set(MediaType::Json);
        //         println!("{}  ROUTE   Sending Pending Admin Approval For ({})", common::local_timestamp(), &login_user.username);
        //         json::encode(&return_status).expect("Failed to serialize response")
        //     } else {
        //         let token: String = auth::issue_initial_token(&user_details, request).expect("FAULT: authenticate.rs add_route()");
        //
        //         let return_status = to_json_status_token(true, "Enjoy your token!", &token);
        //
        //         response.set(StatusCode::Accepted);
        //         response.set(MediaType::Json);
        //         println!("{}  ROUTE   Sending Token For ({})", common::local_timestamp(), &login_user.username);
        //         json::encode(&return_status).expect("Failed to serialize response")
        //     }
        // } else {
        //     let return_status = to_json_status(false, "Invalid password!");
        //
        //     response.set(StatusCode::NotAcceptable);
        //     response.set(MediaType::Json);
        //     println!("{}  ROUTE   Sending Invalid Password For ({})", common::local_timestamp(), &login_user.username);
        //     json::encode(&return_status).expect("Failed to serialize response")
        // }
