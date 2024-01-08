fn main() -> anyhow::Result<()> {
    tonic_build::compile_protos("proto/judge/v1/judge.proto")?;
    Ok(())
}
