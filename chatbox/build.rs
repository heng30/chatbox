use cmd_lib::run_fun;

fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    let _ = write_app_version();
}

fn write_app_version() -> Result<(), Box<dyn std::error::Error>> {
    let tags = run_fun!(git describe --tags --abbrev=0)?
        .split(char::is_whitespace)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    if let Some(version) = tags.last() {
        let output = format!(r#"pub static VERSION: &str = "{}";"#, version);
        let _ = std::fs::write("src/version.rs", output);
    }

    Ok(())
}
