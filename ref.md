## build

```bash
cargo install protobuf-codegen
cargo install grpcio-compiler

protoc --rust_out=src/protos --grpc_out=src/protos --plugin=protoc-gen-grpc=`which grpc_rust_plugin` protos/hello.proto
```
