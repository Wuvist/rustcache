## build

```bash
cargo install protobuf-codegen
cargo install grpcio-compiler

# protoc --rust_out=src --grpc_out=src --plugin=protoc-gen-grpc=C:\Users\wuvis\.cargo\bin\grpc_rust_plugin.exe protos/groupcache.proto
protoc --rust_out=src --grpc_out=src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` protos/groupcache.proto
```
