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
extern crate mylib;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;

// use mylib::types::SqlSafe;
use mylib::types::{LoginUser};
use mylib::errors::{ServerError, LoginError};
use mylib::req_logger;
use nickel::{Nickel, HttpRouter, JsonBody, Router, Request, Response};
use nickel::{Action, NickelError};
use hyper::net::{Openssl, Streaming};
// use std::borrow::Cow;
use std::error::Error as StdError;
use std::net::{SocketAddrV4, Ipv4Addr};
use rustc_serialize::json::{DecoderError, ParserError};
use std::any::Any;
use std::io::{self, ErrorKind};

lazy_static! {
    static ref IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    static ref PORT: u16 = 8080;
    static ref ADDRESS: SocketAddrV4 = SocketAddrV4::new(*IP, *PORT);
}

pub fn get_user<'a>(req: &'a mut Request) -> Result<LoginUser<'a>, ServerError>{
     let user = try!(req.json_as::<LoginUser>());
     Ok(user)
}

pub fn send_error<'mw>(res: Response<'mw>, err: ServerError) ->
    Result<Action<Response<'mw>, Response<'mw, (), Streaming>>, NickelError<'mw>> {
    let top_error: ServerError = err;
    println!("top error {:?}", top_error);
    let second_error = match top_error {
        ServerError::Io(e) => e,
        _ => {
            println!("prolly shouldn't have matched here");
            io::Error::new(ErrorKind::Other, "bla")
        },
    };
    // let second_error = top_error.get_ref().unwrap();
    println!("second error {:?}", second_error);
    let third_error: &StdError = second_error.get_ref().expect("Expecting there to be a nested error here");
    println!("third error {:?}", third_error);
    //
    // let fourth_error = third_error;
    // println!("fourth_error {:?}", fourth_error);
    // match &err {
    //     &ServerError::DecoderError(ref e) => match e {
    //         _ => println!("Unknown String error 1 {:?}", e),
    //     },
    //     &ServerError::Io(ref e) => match e.get_ref().unwrap() {
    //         Err(e1) => match e1.cause() {
    //
    //         },
    //         &DecoderError::ParseError(err) => println!("Unknown String error 2 {:?}", err),
    //         _ => println!("Unknown String error 2 {:?}", e),
    //     },
    //     _ => println!("Unknown Error"),
    // }
    res.send(format!("err {:?}", second_error))
}

fn main() {
    // let user = LoginUser {
    //     // username: "Robert'); DROP TABLE Students;--?",
    //     username: Cow::from("Bobby Tables"),
    //     password: Cow::from("Secret Squirrel"),
    // };

    // match user.username.is_sql_safe() {
    //     Ok(name) => println!("Username \"{}\" is safe", name),
    //     Err(err) => {
    //         println!("Error: Username \"{}\" is NOT SQL safe!", user.username);
    //         println!("Error: {}", err);
    //         println!("Error: {}", err.description());
    //     },
    // };

    let mut server = Nickel::new();
    let mut router: Router = Nickel::router();

    server.utilize(req_logger::logger_fn);

    router.post("/api/authenticate", middleware! { |request, response|
        println!("Request received");
        // let login_user = try_with!(response, {
        //     request.json_as::<FullUser>().map_err(|e| (StatusCode::BadRequest, e))
        // });
        println!("get_user()");
        let user = match get_user(request) {
            Ok(user) => user,
            Err(err) => {
                return send_error(response, err);
            },
        };
        // let user = get_user(request)
        //             .map_err(|err| return send_error(response, err));
        // let user = get_user(request)
        //             .map_err(move |err| { send_error( response, err) } );
        println!("get_user() done");
        // let user = match request.json_as::<LoginUser>() {
        //     Ok(user) => user,
        //     Err(err) => panic!("{:?}", err),
        // };
        println!("{:?}", user);

        // println!("{:?}", login_user);

        // let mut valid_username = false;
        // let mut valid_password = false;

        // match login_user.username {
        //     Some(username) => {
        //         string_is_safe(&username);
        //         empty_string(&username);
        //         valid_username = true;
        //     },
        //     None => valid_username = false,
        // }

        // match login_user.password {
        //     Some(password) => {
        //         empty_string(&password);
        //         valid_password = true;
        //     },
        //     None => valid_password = false,
        // }
        //
        // if !valid_username {
        //     println!("{}  ROUTE   Sending Invalid Username For ({:?})", common::local_timestamp(), &login_user.username);
        //     invalid_username(response);
        // }
        //
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
    });

    server.utilize(router);
    let ssl = Openssl::with_cert_and_key("keys/server.crt", "keys/server.key").expect("Failed to open SSL keys from ./keys/");
    // let listening = server.listen_https(address, ssl).expect("Failed to launch server");
// println!("Listening on: {:?}", listening.socket());

    let listening = match server.listen_https(*ADDRESS, ssl) {
        Ok(srv) => srv,
        Err(err) => panic!("Failed to start Nickel Server, {:?}", err),
    };
    println!("Listening on: {:?}", listening.socket());
}
