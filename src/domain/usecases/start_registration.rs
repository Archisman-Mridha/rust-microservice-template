use anyhow::{Result, anyhow};
use crate::utils;
use super::Usecases;
use derive_more::Constructor;
use validator::{Validate, ValidationError};

#[derive(Constructor, Validate)]
pub struct StartRegistrationRequest<'a> {

  #[validate(email)]
  email: &'a str,

  #[validate(length(min=4, max=50), custom= "usernameValidator")]
  username: &'a str,

  #[validate(length(min=6, max=50))]
  password: &'a str
}

impl Usecases {
  pub async fn startRegistration(&self, args: StartRegistrationRequest<'_>) -> Result<( )> {
    if let Err(errors)= args.validate( ) {
      return Err(anyhow!(utils::extractValidationErrorMessages(errors)));
    }

    // TODO: implement

    return Ok(( ));
  }
}

// usernameValidator validates the given username.
fn usernameValidator(username: &str) -> Result<( ), ValidationError> {
  let mut alphabetCount= 0;

  for character in username.chars( ) {
    // Allow only English alphabets, numbers and underscores.
    if !character.is_alphabetic( ) && !character.is_numeric( ) && character != '_' {
      return Err(ValidationError::new("Only English alphabets, numbers and underscores are allowed in the username"));
    }

    if character.is_alphabetic( ) {
      alphabetCount += 1;
    }
  }

  // Atleast 1 alphabet must be present in the username.
  if alphabetCount == 0 {
    return Err(ValidationError::new("Username cannot contain only underscores and digits"));
  }

  return Ok(( ));
}