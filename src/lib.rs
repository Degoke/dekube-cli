use log::{info, warn};
use rusqlite::{Connection};
use anyhow::{Result, anyhow};

#[derive(Debug)]
struct User {
    id: i32,
    email: String,
    token: Option<String>
}

pub fn authenticate_user(email: &str, password: &str, conn: &Connection) {
    info!("login {} - {}", email, password);

    let current_user: Option<User>;

    match conn.execute("INSERT INTO user (email, token) values (?1, ?2)", &[email, password]) {
        Ok(result) => result,
        Err(error) => panic!("Error inserting data {}", error)
    };

    match check_if_authenticated(&conn) {
        Ok(user) => current_user = Some(user),
        Err(_) => current_user = None
    }

    info!("current user {:?}", current_user);

    match current_user {
        Some(user) => info!("welcome user {}", user.email),
        None => warn!("user not authenticated")
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