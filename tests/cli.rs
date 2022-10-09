use assert_cmd::Command;

const VALID_INPUT_PATH: &str = "tests/rustonomicon.json";
const MISSING_INPUT_PATH: &str = "tests/file-does-not-exist.json";
const INVALID_INPUT_PATH: &str = "tests/invalid_bookcision.json";

#[test]
fn default_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("highlights")?;
    let output_file = assert_fs::NamedTempFile::new("rustonomicon.md")?;

    cmd.arg(VALID_INPUT_PATH).arg(output_file.path());
    cmd.assert().success();

    Ok(())
}

#[test]
fn stdout_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("highlights")?;

    cmd.arg(VALID_INPUT_PATH);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("# Rustonomicon"));

    Ok(())
}

#[test]
fn missing_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("highlights")?;

    cmd.arg(MISSING_INPUT_PATH);
    cmd.assert()
        .failure()
        .code(74)
        .stderr(predicates::str::contains("cannot read input file"))
        .stderr(predicates::str::contains("file-does-not-exist.json"));

    Ok(())
}

#[test]
fn invalid_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("highlights")?;

    cmd.arg("tests/rustonomicon.json")
        .arg("missing_dir/missing_file.md");
    cmd.assert()
        .failure()
        .code(74)
        .stderr(predicates::str::contains("cannot write to file"))
        .stderr(predicates::str::contains("missing_dir"))
        .stderr(predicates::str::contains("missing_file.md"));

    Ok(())
}

#[test]
fn invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("highlights")?;
    let output_file = assert_fs::NamedTempFile::new("rustonomicon.md")?;

    cmd.arg(INVALID_INPUT_PATH).arg(output_file.path());
    cmd.assert()
        .failure()
        .code(65)
        .stderr(predicates::str::contains("invalid bookcision json file"));

    Ok(())
}
