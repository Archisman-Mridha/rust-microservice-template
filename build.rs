#![allow(non_snake_case)]

use std::path::Path;

const PROTO_FILES_DIR: &str = "src/cmd/server/grpc/api/proto";

fn main() {
  let protoFilePath = Path::new(PROTO_FILES_DIR).join("service.proto");

  /*
    tonic-build generates a descriptor file corresponding to the service.proto file.
    prost then generates necessary Rust code using that descriptor file.

    Descriptors are the commonly used language model for Protocol Buffers. They are used as an
    intermediate artifact to support code generation, and they are also used in runtime
    libraries to implement support for reflection and dynamic types.
    REFERENCE : https://protobuf.com/docs/descriptors.
  */
  tonic_build::configure()
    .build_client(false)
    .compile_protos(&[protoFilePath], &[PROTO_FILES_DIR])
    .expect("Failed compiling proto files");
}
