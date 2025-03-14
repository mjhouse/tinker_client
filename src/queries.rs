use reqwest::StatusCode;
use serde::{Serialize,Deserialize};
use crate::errors::{Error, Result};

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
    pub id: i32,
    pub username: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AccountKey {
    pub id: i32,
    pub name: String,
    pub token: String,
}

pub fn register<T: ToString>(username: T, password1: T, password2: T) -> Result<AccountInfo> {
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
        .send()?;

    if response.status().is_success() {
        let text = response.text()?;
        Ok(serde_json::from_slice(text.as_bytes())?)
    } else {
        Err(Error::RegisterFailed)
    }
}

pub fn login<T: ToString>(username: T, password: T) -> Result<AccountKey> {
    const URL: &str = "http://localhost:8080/login";

    let username = username.to_string();
    let password = password.to_string();
    
    let client = reqwest::blocking::Client::new();
    
    let response = client.get(URL)
        .json(&LoginForm {
            username,
            password,
        })
        .send()?;

    if response.status().is_success() {
        let text = response.text()?;
        Ok(serde_json::from_slice(text.as_bytes())?)
    } else {
        Err(Error::LoginFailed)
    }
}