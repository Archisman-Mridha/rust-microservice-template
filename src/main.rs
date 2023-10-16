#![allow(non_snake_case)]

mod config;
mod utils;
mod domain;
mod adapters;

mod proto {
  // Including code generated from the .proto files.

  tonic::include_proto!("authentication.service");

  // Descriptors are the commonly used language model for Protocol Buffers. They are used as an
  // intermediate artifact to support code generation, and they are also used in runtime libraries
  // to implement support for reflection and dynamic types.
  // Read more here - https://protobuf.com/docs/descriptors
  pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("authentication.service.descriptor");
}

use std::process::exit;
use opentelemetry::global::shutdown_tracer_provider;
use utils::THREAD_CANCELLATION_TOKEN;
use adapters::{GrpcAdapter, OpentelemetryAdapter};
use domain::Usecases;
use tokio::{signal, spawn};

#[tokio::main]
async fn main( ) -> Result<( ), ( )> {

  let usecases: &'static Usecases= &Usecases{ };

  OpentelemetryAdapter::initTraceExporter( );
  OpentelemetryAdapter::initMetricsExporter( );

  let grpcAdapter= &GrpcAdapter{ };

  spawn(async move {
    grpcAdapter.startServer(usecases).await;
  });

  /* Gracefully shutdown on receiving program shutdown signal. */ {
    let error= signal::ctrl_c( ).await.err( );
    println!("Received program shutdown signal");

    let _ =&THREAD_CANCELLATION_TOKEN.cancel( ); // Do cleanup tasks in currently active Tokio
                                                 // threads.
    shutdown_tracer_provider( );

    match error {
      None => exit(0),

      Some(error) => {
        println!("Error: {}", error);
        exit(1);
      }
    }
  }
}