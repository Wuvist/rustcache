## build

```bash
cargo install protobuf-codegen
cargo install grpcio-compiler

# protoc --rust_out=src/protos --grpc_out=src/protos --plugin=protoc-gen-grpc=C:\Users\wuvis\.cargo\bin\grpc_rust_plugin.exe proto/groupcache.proto
protoc --rust_out=src/protos --grpc_out=src/protos --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/groupcache.proto


protoc --go_out=plugins=grpc:goclient proto/groupcache.proto

$env:Path += ";C:\tools\protoc\bin"
setx /M PATH "%PATH%;C:\tools\protoc\bin"
```
