# Rust Microservice Template

Command used to generate self-signed private key and TLS certificate (for encryption at flight for
the gRPC server) :
```bash
mkdir -p tests/tls/ && \
  cd tests/tls && \
  openssl \
    req -newkey rsa:2048 \
    -new -nodes -x509 -days 3650 \
    -keyout private-key.pem -out certificate.pem
```

Using `Autometrics` - https://fiberplane.com/blog/adding-observability-to-rust-grpc-services-using-tonic-and-autometrics