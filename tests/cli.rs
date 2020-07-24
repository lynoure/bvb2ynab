use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bvb2ynab")?;

    cmd.arg("test/file/is/not/there");
    cmd.assert().failure().stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn conversion_from_sample() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bvb2ynab")?;

    cmd.arg("example-file-in-utf8.csv");
    cmd.assert().success().stdout(predicate::str::contains("Date,Payee,Memo,Amount"));
    cmd.assert().success().stdout(predicate::str::contains("23.06.2020,HUMBLEBUNDL,Basislastschrift PP.5058.PP . HUMBLEBUNDL I hr Einkauf bei HUMBLEBUNDL EREF: 1009395378020 PP.5058 .PP PAYPAL MREF: 5VRJ224NEA DVG CRED: LU96ZZZ0000000000 000000058 IBAN: DE885007001 00175526303 BIC: DEUTDEFF,-28.00"));

    Ok(())
}