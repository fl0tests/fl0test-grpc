//! Build file for performing code-gen pass on the proto files.

fn main() {
    tonic_build::compile_protos("proto/canary.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {e:?}"));
}
