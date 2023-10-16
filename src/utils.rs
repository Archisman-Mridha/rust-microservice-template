use lazy_static::lazy_static;
use validator::ValidationErrors;
use tokio_util::sync::CancellationToken;

lazy_static! {
  // This cancellation token will be activated when the program receives a shutdown signal. It will
  // trigger cleanup tasks in active Tokio threads.
  pub static ref THREAD_CANCELLATION_TOKEN: CancellationToken= CancellationToken::new( );
}

pub static SERVER_ERROR: &'static str= "Server error occurred";

// extractValidationErrorMessages takes in a value of type ValidationErrors and extracts the error
// messages into a vector. Those error messages are joined by ' | ' and the final string is returned
// back.
pub fn extractValidationErrorMessages(error: ValidationErrors) -> String {
  return error.field_errors( )
    .into_iter( )
    .map(|item| -> &str {
      let mut errorMessage: &str= "";

      let fieldValidationErrors= item.1;
      for error in fieldValidationErrors {
        if let std::borrow::Cow::Borrowed(code)= error.code {
          match code {
            "email" => errorMessage= "Email is invalid",

            "length" => errorMessage= "Password should be between 6 - 50 characters",

            code => errorMessage= code
          }
        }
      }

      return errorMessage;
    })
    .collect::<Vec<&str>>( )
    .join(" | ");
}