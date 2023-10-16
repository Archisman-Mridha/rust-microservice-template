use super::Usecases;
use anyhow::Result;
use crate::proto::*;

impl Usecases {
  pub async fn verifyEmail(&self, args: &VerifyEmailRequest) -> Result<AuthenticationResponse> {

    unimplemented!( )
  }
}