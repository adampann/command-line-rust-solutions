use assert_cmd::Command; // Import Command

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("hello").unwrap(); // create Command to run hello in current
                                                        // crate. Returns a Result & then calls
                                                        // Result::unwrap.
                                                        // If binary not found unwrap will fail &
                                                        // fail test.
    cmd.assert().success(); // This ensures the test will command will succeed if nothing bad
                            // happens.
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
} 

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n"); //Verfies that the command exits successfully
                                                      //& pritns the expected text to STDOUT
}
