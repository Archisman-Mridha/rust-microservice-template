use std::env;
use lazy_static::lazy_static;

pub struct Config {
  pub GRPC_PORT: String,
  pub TLS_DIR: String,

  pub JAEGER_COLLECTOR_URL: String,
  pub METRICS_SERVER_PORT: String
}

impl Config {
  // init loads environment variables from the .env file. An instance of the Config struct is
  // constructed using those envs and returned back.
  fn init( ) -> Self {
    dotenv::from_filename(".env")
      .expect("Error loading .env file");

    return Self {
      GRPC_PORT: getEnv("GRPC_PORT"),
      TLS_DIR: getEnv("TLS_DIR"),

      JAEGER_COLLECTOR_URL: getEnv("JAEGER_COLLECTOR_URL"),
      METRICS_SERVER_PORT: getEnv("METRICS_SERVER_PORT")
    };
  }
}

lazy_static! {
  // This value is initialized (in a thread safe manner) on the heap, when it is accessed for the
  // first time.
  // Read more about lazy_static here - https://blog.logrocket.com/rust-lazy-static-pattern/
  pub static ref CONFIG: Config= Config::init( );
}

// getEnv fetches the given environment variable.
fn getEnv(name: &str) -> String {
  return env::var(name)
    .expect(&format!("Error getting env {}", name));
}