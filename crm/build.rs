use std::fs;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["../protos/crm.proto"], &["../protos"])?;

    Ok(())
}
