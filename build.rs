fn main() {
    tonic_build::compile_protos("sample.proto").unwrap();
}
