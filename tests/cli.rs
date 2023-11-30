use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bibget")?;
    cmd.arg("10.1002/sim.1186");
    cmd.assert().success().stdout(predicate::str::contains(
        "title = {Quantifying heterogeneity in a meta‐analysis},",
    ));
    Ok(())
}
