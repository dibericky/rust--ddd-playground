use anyhow::{Error, Result};
use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
struct Email(String);
#[derive(Debug)]
struct VerifiedEmail(Email);
#[derive(Debug)]
struct UnverifiedEmail(Email);

#[derive(Debug)]
struct Age(i32);

#[derive(Debug)]
enum UserEmail {
    VerifiedEmail(VerifiedEmail),
    UnverifiedEmail(UnverifiedEmail),
}

#[derive(Debug)]
struct User {
    name: String,
    middle_name: Option<String>,
    surname: String,
    age: Age,
    email: UserEmail,
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl User {
    fn new(
        name: String,
        middle_name: Option<String>,
        surname: String,
        age: Age,
        email: Email,
    ) -> Self {
        Self {
            name,
            middle_name,
            surname,
            age,
            email: UserEmail::UnverifiedEmail(UnverifiedEmail(email)),
        }
    }
}

fn verify_email(email: &UnverifiedEmail) -> Result<VerifiedEmail> {
    let UnverifiedEmail(unverified_email) = email;

    let is_ok = unverified_email.0.contains("ok");
    // verify email
    if is_ok {
        Ok(VerifiedEmail(Email(unverified_email.0.clone())))
    } else {
        Err(Error::msg("Email has not been verified yet"))
    }
}

fn check_email(email: String) -> Result<Email> {
    let re = Regex::new(r"^[\w.]+@[\w.]+\.\w+$").unwrap();
    if re.is_match(&email) {
        Ok(Email(email))
    } else {
        Err(Error::msg("Invalid email"))
    }
}

fn check_age(age: i32) -> Result<Age> {
    match age {
        x if x < 0 => Err(Error::msg("Age cannot be negative")),
        x if x < 13 => Err(Error::msg(
            "Sorry but this service is unavailable for minor of 13 years old",
        )),
        x if x > 120 => Err(Error::msg("I don't think you can be immortal")),
        _ => Ok(Age(age)),
    }
}

fn create_user(
    email: String,
    age: i32,
    name: String,
    surname: String,
    middle_name: Option<String>,
) -> Result<User> {
    let age = check_age(age)?;
    let email = check_email(email)?;

    let user = User::new(name, middle_name, surname, age, email);

    Ok(user)
}

fn grant_user(user: &mut User) -> Result<()> {
    if let UserEmail::UnverifiedEmail(unverified_email) = &user.email {
        let verified_email = verify_email(unverified_email)?;
        user.email = UserEmail::VerifiedEmail(verified_email);
    }
    Ok(())
}

fn get_fullname(user: &User) -> String {
    let middle_name = user.middle_name.as_ref().map(|middle| middle.to_owned());
    vec![
        Some(user.name.to_owned()),
        middle_name,
        Some(user.surname.to_owned()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ")
}

fn main() -> Result<()> {
    let input_email = "foo@ok.com".to_string();
    let input_age = 22;
    let name = "Luca".to_string();
    let surname = "Rossi".to_string();
    let middle_name: Option<String> = None;

    let mut user = create_user(input_email, input_age, name, surname, middle_name)?;

    let fullname = get_fullname(&user);

    println!("Welcome {} of {} years old", fullname, user.age.0);

    grant_user(&mut user)?;
    if let UserEmail::VerifiedEmail(verified_email) = user.email {
        println!("User email {} is verified!", verified_email.0);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_create_user() {
        let input_email = "foo@ok.com".to_string();
        let input_age = 22;
        let name = "Luca".to_string();
        let surname = "Rossi".to_string();
        let middle_name: Option<String> = None;

        let user = create_user(input_email, input_age, name, surname, middle_name);
        assert!(user.is_ok());
        let mut user = user.unwrap();
        let result = grant_user(&mut user);
        assert!(result.is_ok());

        assert_eq!(user.name, "Luca".to_string());
        assert_eq!(user.surname, "Rossi".to_string());
        assert!(user.middle_name.is_none());
        assert_eq!(user.age.0, 22);

        let is_verified_email = match user.email {
            UserEmail::VerifiedEmail(_) => true,
            UserEmail::UnverifiedEmail(_) => false,
        };
        assert!(is_verified_email);
    }

    #[test]
    fn ok_create_user_unverified() {
        let input_email = "foo@unverified.com".to_string();
        let input_age = 22;
        let name = "Luca".to_string();
        let surname = "Rossi".to_string();
        let middle_name: Option<String> = None;

        let user = create_user(input_email, input_age, name, surname, middle_name);
        let mut user = user.unwrap();
        let result = grant_user(&mut user);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Email has not been verified yet");

        let is_unverified_email = match user.email {
            UserEmail::VerifiedEmail(_) => false,
            UserEmail::UnverifiedEmail(_) => true,
        };
        assert!(is_unverified_email);
    }

    #[test]
    fn err_invalid_email() {
        let input_email = "foo.at.com".to_string();
        let input_age = 22;
        let name = "Luca".to_string();
        let surname = "Rossi".to_string();
        let middle_name: Option<String> = None;

        let user = create_user(input_email, input_age, name, surname, middle_name);

        assert!(user.is_err());
        let error = user.unwrap_err();
        assert_eq!(error.to_string(), "Invalid email");
    }

    #[test]
    fn err_invalid_age_negative() {
        let input_email = "fo@ok.com".to_string();
        let input_age = -100;
        let name = "Luca".to_string();
        let surname = "Rossi".to_string();
        let middle_name: Option<String> = None;

        let user = create_user(input_email, input_age, name, surname, middle_name);

        assert!(user.is_err());
        let error = user.unwrap_err();
        assert_eq!(error.to_string(), "Age cannot be negative");
    }

    #[test]
    fn err_invalid_age_immortal() {
        let input_email = "fo@ok.com".to_string();
        let input_age = 130;
        let name = "Luca".to_string();
        let surname = "Rossi".to_string();
        let middle_name: Option<String> = None;

        let user = create_user(input_email, input_age, name, surname, middle_name);

        assert!(user.is_err());
        let error = user.unwrap_err();
        assert_eq!(error.to_string(), "I don't think you can be immortal");
    }
}
