fn main() {
    tonic_build::prost::compile_protos("proto/user.proto").unwrap();
}
