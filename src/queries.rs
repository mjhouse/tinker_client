use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct RegisterForm {
    pub username: String,
    pub password1: String,
    pub password2: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AccountInfo {
    id: i32,
    name: String,
    character_id: Option<i32>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AccountKey {
    id: i32,
    name: String,
    token: String,
}

pub fn register<T: ToString>(username: T, password1: T, password2: T) -> AccountInfo {
    const URL: &str = "http://localhost:8080/register";

    let username = username.to_string();
    let password1 = password1.to_string();
    let password2 = password2.to_string();
    
    let client = reqwest::blocking::Client::new();
    
    let response = client.post(URL)
        .json(&RegisterForm {
            username,
            password1,
            password2
        })
        .send();

    let text = response.unwrap().text().unwrap();
    serde_json::from_slice(text.as_bytes()).unwrap()
}

pub fn login<T: ToString>(username: T, password: T) -> AccountInfo {
    const URL: &str = "http://localhost:8080/login";

    let username = username.to_string();
    let password = password.to_string();
    
    let client = reqwest::blocking::Client::new();
    
    let response = client.get(URL)
        .json(&LoginForm {
            username,
            password,
        })
        .send();

    let text = response.unwrap().text().unwrap();
    serde_json::from_slice(text.as_bytes()).unwrap()
}