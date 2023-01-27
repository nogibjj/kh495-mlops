use assert_cmd::Command;

#[test]
// Test the add function
fn test_add() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.arg("calc")
        .arg("--x")
        .arg("5")
        .arg("--y")
        .arg("3")
        .arg("--op")
        .arg("+");
    cmd.assert().success().stdout("Result: 8.00\n");
}

#[test]
// Test the subtract function
fn test_subtract() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.arg("calc")
        .arg("--x")
        .arg("5")
        .arg("--y")
        .arg("3")
        .arg("--op")
        .arg("-");
    cmd.assert().success().stdout("Result: 2.00\n");
}

#[test]
// Test the divide function
fn test_divide() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.arg("calc")
        .arg("--x")
        .arg("5")
        .arg("--y")
        .arg("3")
        .arg("--op")
        .arg("/");
    cmd.assert().success().stdout("Result: 1.67\n");
}

#[test]
// Test the multiply function
fn test_multiply() {
    let mut cmd = Command::cargo_bin("calculator").unwrap();
    cmd.arg("calc")
        .arg("--x")
        .arg("5")
        .arg("--y")
        .arg("3")
        .arg("--op")
        .arg("x");
    cmd.assert().success().stdout("Result: 15.00\n");
}
