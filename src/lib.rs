use log::{info, warn, error};
use reqwest::blocking::Response;
use reqwest::{Error};
use rusqlite::{Connection};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    email: String,
    token: Option<String>
}

#[derive(Serialize, Deserialize)]
struct UserLoginRequest {
    user: UserLoginDto
}

#[derive(Serialize, Deserialize)]
struct UserLoginDto {
        email: String,
        password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginResponse {
    user: User
}

pub fn authenticate_user(email: &str, password: &str, conn: &Connection) {
    info!("login {}", email);

    let current_user: Option<User>;
    let request_url = "http://localhost:8000/api/users/login";

    let new_dto = UserLoginDto {
        email: email.into(),
        password: password.into()
    };

    let new_login: UserLoginRequest = UserLoginRequest {
        user: new_dto.into()
    };

    // let new_login = serde_json::to_string(&new_login).unwrap();
    // info!("{:?}", new_login);

    let client = reqwest::blocking::Client::new();

    let res = client.post(request_url).json(&new_login).send();

    match handle_auth_response(res) {
        Ok(res) => {
            info!("{:?}", res);
            match get_user_by_email(conn, &res.user.email) {
                Ok(user) => match conn.execute("UPDATE user SET token = (?1) WHERE id = (?2)", (&res.user.token, user.id)) {
                    Ok(result) => result,
                    Err(error) => panic!("Error updating data {}", error)
                },
                Err(error) => {
                    warn!("error {}", error);
                    match conn.execute("INSERT INTO user (email, token) values (?1, ?2)", (&res.user.email, &res.user.token)) {
                    Ok(result) => result,
                    Err(error) => panic!("Error inserting data {}", error)
                }
            }
            };
            
        },
        Err(error) => error!("Error authenticating user{:?}", error)
    };

    match check_if_authenticated(&conn) {
        Ok(user) => current_user = Some(user),
        Err(_) => current_user = None
    }

    match current_user {
        Some(user) => info!("welcome {}", user.email),
        None => warn!("user not authenticated")
    }

}

fn handle_auth_response(res: Result<Response, Error>) -> Result<UserLoginResponse> {
    match res {
        Ok(res) => {
            if res.status() == 200 {
                return Ok(res.json::<UserLoginResponse>()?)
            } else {
                return Err(anyhow!("{:?}", res.bytes()))
            }
            
    },
        Err(error) => panic!("Error authenticating user {}", error)
    }
}


fn check_if_authenticated(conn: &Connection) -> Result<User> {
    let mut stmt = conn.prepare("SELECT id, email, token FROM user")?;

    let mut user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            token: row.get(2)?,
        })

    })?;

    match user_iter.nth(0) {
        Some(user) => return Ok(user?),
        None => return Err(anyhow!("User not found"))
    };

}

fn get_user_by_email(conn: &Connection, email: &String) -> Result<User, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, email, token FROM user WHERE email = (?1)")?;

    let mut user_iter = stmt.query_map([email], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            token: row.get(2)?,
        })

    })?;

    // info!("user exists {:?}", user_iter.nth(0).unwrap());
   

    // match user_iter.nth(0) {
    //     Some(Ok(user)) => return Ok(user),
    //     None => return Err(anyhow!("User not found")),
    //     _ => return Err(anyhow!("User not found")),
    // };

    return user_iter.nth(0).unwrap();
}

// /**
//  * create app
//  * should be logged in
//  * collect name and create app

//  */

// // /**
// //  * upload
// //  * should be logged in 
// //  * collect path to dockerfile app-name
// //  * upload dockerfile
// //  */

// /**
//  * deploy
//  */