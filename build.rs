fn main() {
    prost_build::compile_protos(
        &["proto/transaction.proto"], // your proto files
        &["proto"],                   // directories to use as --proto_path
    )
    .unwrap();
}
