
use std::collections::BTreeMap;

fn main() {
    let vars: BTreeMap<&str, Vec<&str>> = [
        ("BUILT_BY_USER",   vec!["whoami"]),
        ("BUILT_BY_HOST",   vec!["hostname"]),
        ("BUILD_TIMESTAMP", vec!["date", "+%H:%M:%S %d-%m-%Y"]),
        ("BUILD_COMPILER",  vec!["rustc", "--version"]),
        ("BUILD_COMMIT",    vec!["git", "log", "-n", "1", "--pretty=format:%h"]),
    ].iter().cloned().collect();

    for (var, cmd) in vars {
        let mut cmd = cmd.iter();

        let command = std::process::Command::new(cmd.next().unwrap()).args(cmd).output().expect("Failed to execute process");
        let output = String::from_utf8(command.stdout).expect("Invalid output");

        println!("cargo:rustc-env={}={}", var, output);
    }
}
