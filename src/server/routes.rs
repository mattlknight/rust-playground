use hyper::status::{StatusCode};
use nickel::{HttpRouter, Router, MediaType};

use ::traits::{SqlSafe};
use ::types::{LoginUser};
use ::server::common::{get_user, send_json_error, json_reply};


pub fn add_route(router: &mut Router) {
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
        let encoded = json_reply(Some(&format!("Welcome {}", user.username)), None);
        response.set(StatusCode::Ok);
        response.set(MediaType::Json);
        (encoded)
    });
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
