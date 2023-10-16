use std::{time::Duration, path::PathBuf, fs, str::FromStr};
use anyhow::Error;
use autometrics::autometrics;
use tokio::{spawn, time::interval};
use tonic::Code;
use crate::proto::{*, authentication_service_server::*};
use crate::utils::{SERVER_ERROR, THREAD_CANCELLATION_TOKEN};
use crate::{config::CONFIG, domain::Usecases};
use tonic::{
  codec::CompressionEncoding,
  async_trait,
  Status,
  Response,
  Request,
  transport::{Server, ServerTlsConfig, Identity}
};
use tracing::{instrument, event, Level};

pub struct GrpcAdapter { }

const MAX_REQUEST_SIZE: usize= 512; //bytes

impl GrpcAdapter {

  // startServer creates a gRPC server and starts it at the desired network address (provided
  // via an environment variable).
  pub async fn startServer(&self, usecases: &'static Usecases) {
    let address= format!("[::]:{}", &*CONFIG.GRPC_PORT);
    let address= address.parse( )
                        .expect(&format!("Error parsing binding address of the gRPC server : {}", address));

    let authenticationService= AuthenticationServiceServer::new(AuthenticationServiceImpl{ usecases })
      .max_decoding_message_size(MAX_REQUEST_SIZE)
      // Read more about the compression feature - https://grpc.io/docs/guides/compression/.
      .send_compressed(CompressionEncoding::Gzip)
      .accept_compressed(CompressionEncoding::Gzip);

    // Support for gRPC server health-checking.
    let (mut healthReporter, healthcheckService)= tonic_health::server::health_reporter( );
    spawn(async move {
      let mut ticker= interval(Duration::from_secs(5));
      loop {
        ticker.tick( ).await;

        let servingStatus= tonic_health::ServingStatus::Serving;
        healthReporter.set_service_status("AuthenticationService", servingStatus).await;
      }
    });

    // Adding gRPC server reflection capabilities.
    let reflectionService= tonic_reflection::server::Builder::configure( )
      .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
      .build( )
      .expect("Error building gRPC reflection service")
      .max_decoding_message_size(MAX_REQUEST_SIZE);

    // Using TLS for encryption at flight.
    let dirWithTlsRelatedFiles= PathBuf::from_str(&*CONFIG.TLS_DIR)
      .expect("Error parsing the TLS directory");
    let (certificateFilepath, privateKeyFilepath)= (

      fs::read_to_string(dirWithTlsRelatedFiles.join("certificate.pem"))
        .expect("Error getting TLS certificate"),

      fs::read_to_string(dirWithTlsRelatedFiles.join("private-key.pem"))
        .expect("Error getting private-key for TLS encryption")
    );
    let tlsConfig= ServerTlsConfig::new( ).identity(
      Identity::from_pem(certificateFilepath, privateKeyFilepath));

    println!("Starting gRPC server");

    Server::builder( )
      .tls_config(tlsConfig).expect("Error enforcing TLS config on the gRPC server")
      .add_service(authenticationService)
      .add_service(healthcheckService)
      .add_service(reflectionService)
      .serve_with_shutdown(address, THREAD_CANCELLATION_TOKEN.clone( ).cancelled( ))
      .await.expect("Error trying to start the gRPC server");
  }
}

#[derive(Debug)]
pub struct AuthenticationServiceImpl {
  usecases: &'static Usecases
}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {

  #[autometrics]
  #[instrument(name = "StartRegistration")]
  async fn start_registration(&self, request: Request<StartRegistrationRequest>) -> Result<Response<( )>, Status> {
    let request= request.into_inner( );

    use crate::domain::StartRegistrationRequest;
    let args=
      StartRegistrationRequest::new(&request.email, &request.username, &request.password);

    return match self.usecases.startRegistration(args).await {
      Ok(_) => Ok(Response::new(( ))),

      Err(error) => {
        let code= getGrpcStatusCode(&error);

        if code == Code::Internal {
          event!(Level::ERROR, %error);
        }

        Err(Status::new(code, error.to_string( )))
      }
    };
  }

  #[autometrics]
  #[instrument(name = "VerifyEmail")]
  async fn verify_email(&self, request: Request<VerifyEmailRequest>) -> Result<Response<AuthenticationResponse> ,Status> {
    let request= request.into_inner( );

    return match self.usecases.verifyEmail(&request).await {
      Ok(output) => Ok(Response::new(output)),

      Err(error) => {
        event!(Level::ERROR, %error);
        Err(Status::new(Code::Internal, SERVER_ERROR))
      }
    };
  }
}

// getGrpcStatusCode takes an anyhow error and returns an appropriate gRPC status code by analysing
// the error.
fn getGrpcStatusCode(error: &Error) -> Code {
  if error.to_string( ) == SERVER_ERROR {
    return Code::Internal;
  }

  else { return Code::InvalidArgument; }
}