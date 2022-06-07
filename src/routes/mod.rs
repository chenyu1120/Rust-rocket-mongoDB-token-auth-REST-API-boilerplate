pub mod authorization;
pub mod routes_models;
pub mod validator_authorization;

pub enum TypeValidDataFromRegistration {
    Ok,
    BadFirstName,
    BadLastName,
    BadLogin,
    BadPassword,
    BadMail,
}

pub enum TypeValidTwoStr {
    Ok,
    BadFirst,
    BadSecond,
}

pub enum TypeValidMail {
    Ok,
    BadMail,
}
