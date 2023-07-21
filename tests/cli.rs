use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn temp() -> TestResult {
let expected = "Temperature in degree celsius: 37.77778\n";
let mut cmd = Command::cargo_bin("temp")?;
cmd.arg("100").assert().success().stdout(expected);
Ok(())
}