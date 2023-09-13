// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    let dbc_path = "dbc/e46.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={}", dbc_path);

    let mut out_file = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen("example.dbc", &dbc_file, &mut out_file, true)
}
