#[macro_use] extern crate lazy_static;
extern crate regex;
// extern crate std;

mod sql;
pub mod traits;
pub mod types;
// use std;











// #[derive(Debug)]
// enum LoginError {
//     EmptyUsername,
//     InvalidUsername,
//     EmptyPassword,
//     InvalidPassword,
// }
//
// impl fmt::Display for LoginError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             LoginError::EmptyUsername =>
//                 write!(f, "please provide a non-empty username"),
//             LoginError::InvalidUsername =>
//                 write!(f, "please provide a valid username"),
//             LoginError::EmptyPassword =>
//                 write!(f, "please provide a non-empty password"),
//             LoginError::InvalidPassword =>
//                 write!(f, "please provide a valid password"),
//         }
//     }
// }
//
// impl error::Error for LoginError {
//     fn description(&self) -> &str {
//         match *self {
//             LoginError::EmptyUsername =>
//                 "empty username not allowed",
//             LoginError::InvalidUsername =>
//                 "invalid username not allowed",
//             LoginError::EmptyPassword =>
//                 "empty password not allowed",
//             LoginError::InvalidPassword =>
//                 "invalid password not allowed",
//         }
//     }
//
//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//             LoginError::EmptyUsername => None,
//             LoginError::InvalidUsername => None,
//             LoginError::EmptyPassword => None,
//             LoginError::InvalidPassword => None,
//         }
//     }
// }

// fn string_is_safe(string: &str) -> Result {
//     // TODO: Add regex check for any non alphanumeric characters
//     let invalid_sql_string = Regex::new(r"([^\d\w\s@\.-]+)").expect("FAULT  db/mod.rs string_is_safe()");
//     if Regex::is_match(&invalid_sql_string, string) {
//         let cap = invalid_sql_string.captures(string).expect("FAULT  db/mod.rs string_is_safe()");
//         println!("UNSAFE SQL STRING [{}]", string);
//         panic!("UNSAFE SQL STRING FOUND [{:?}]", cap);
//     }
// }

// fn empty_string(string: &str) {
//     if string.is_empty() {
//         panic!("EMPTY STRING FOUND");
//     }
// }



/*
fn get_login_from_db(db: &PooledConnection<PostgresConnectionManager>,
    username: &str) -> DBUserLogin {

    let query = "SELECT id, password_hash FROM fiplan_auth.users WHERE username=$1";
    let query_params: &[&ToSql] = &[&username];

    select_single_from_db::<DBUserLogin>(&db, &query, &query_params, login_filter)
}

pub fn invalid_username(response: Response) {
    let return_status = to_json_status(false, "Invalid password!");

    response.set(StatusCode::NotAcceptable);
    response.set(MediaType::Json);
    response.send(json::encode(&return_status).expect("Failed to serialize response"));
}

pub fn add_route(router: &mut Router) {
    router.post("/api/auth/authenticate", middleware! { |request, mut response|

        let login_user = try_with!(response, {
            request.json_as::<FullUser>().map_err(|e| (StatusCode::BadRequest, e))
        });
        println!("{:?}", login_user);

        let mut valid_username = false;
        let mut valid_password = false;

        match login_user.username {
            Some(username) => {
                string_is_safe(&username);
                empty_string(&username);
                valid_username = true;
            },
            None => valid_username = false,
        }

        match login_user.password {
            Some(password) => {
                empty_string(&password);
                valid_password = true;
            },
            None => valid_password = false,
        }

        if !valid_username {
            println!("{}  ROUTE   Sending Invalid Username For ({:?})", common::local_timestamp(), &login_user.username);
            invalid_username(response);
        }

        let db = request.pg_conn().expect("Failed to get a connection from pool");
        let db_user = &login_user.username.map(|x| get_login_from_db(&db, &x));


        if &login_user.password.map(|x| auth::validate_password(&x, &db_user.password_hash)) {
            let user_details = get_user_from_db(&db, &db_user.id);

            if user_details.email_verified != Some(true) {
                let return_status = to_json_status(false, "Pending email verification!");

                response.set(StatusCode::NotAcceptable);
                response.set(MediaType::Json);
                println!("{}  ROUTE   Sending Email Not Verified For ({})", common::local_timestamp(), &login_user.username);
                json::encode(&return_status).expect("Failed to serialize response")
            } else if user_details.admin_approved != Some(true) {
                let return_status = to_json_status(false, "Pending admin approval!");

                response.set(StatusCode::NotAcceptable);
                response.set(MediaType::Json);
                println!("{}  ROUTE   Sending Pending Admin Approval For ({})", common::local_timestamp(), &login_user.username);
                json::encode(&return_status).expect("Failed to serialize response")
            } else {
                let token: String = auth::issue_initial_token(&user_details, request).expect("FAULT: authenticate.rs add_route()");

                let return_status = to_json_status_token(true, "Enjoy your token!", &token);

                response.set(StatusCode::Accepted);
                response.set(MediaType::Json);
                println!("{}  ROUTE   Sending Token For ({})", common::local_timestamp(), &login_user.username);
                json::encode(&return_status).expect("Failed to serialize response")
            }
        } else {
            let return_status = to_json_status(false, "Invalid password!");

            response.set(StatusCode::NotAcceptable);
            response.set(MediaType::Json);
            println!("{}  ROUTE   Sending Invalid Password For ({})", common::local_timestamp(), &login_user.username);
            json::encode(&return_status).expect("Failed to serialize response")
        }
    });
}
*/
