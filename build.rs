fn main() -> Result<(), Box<dyn std::error::Error>> {
    //or tonic_build::compile_protos("gRPC/proto/records.proto")?;
    tonic_build::compile_protos("proto/records.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    Ok(())
}