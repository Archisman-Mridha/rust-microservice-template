# Rust Microservice Template

Command used to generate self-signed private key and TLS certificate (for encryption at flight for
the gRPC server) :
```bash
mkdir -p tls/ && \
  cd tls && \
  openssl \
    req -newkey rsa:2048 \
    -new -nodes -x509 -days 3650 \
    -keyout private-key.pem -out certificate.pem
```