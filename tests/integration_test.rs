#[cfg(test)]
use assert_cmd::Command;

#[test]
fn test_invalid_args() {
    let mut cmd = Command::cargo_bin("pyshop").expect("Failed to find the binary");
    cmd.arg("-N").arg("-F");
    cmd.assert()
        .success()
        .stdout("Usage: ./hash_finder -N <number of zeros> -F <number of hashes to find>\n");
}
