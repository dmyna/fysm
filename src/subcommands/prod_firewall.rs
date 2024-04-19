use rusqlite::{Connection, Error as SQLError};
use std::process::Command;

use crate::DB_PATH;

const TABLE_NAME: &str = "fw_rules";

fn add_rule_to_db(
    conn: &Connection,
    sub_domain: &str,
    domain: &String,
    rule: &str,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        format!(
            "INSERT INTO {} VALUES ('{}', '{}', '{}')",
            TABLE_NAME, sub_domain, domain, rule
        )
        .as_str(),
        [],
    )
}
fn set_on_iptables(action_param: &str, domain: &str, rule: &str) -> std::process::Output {
    let output = Command::new("sudo")
        .arg("iptables")
        .arg(format!("-{}", action_param))
        .arg("OUTPUT")
        .arg("-d")
        .arg(&domain)
        .arg("-j")
        .arg(rule)
        .output()
        .unwrap();

    output
}
fn verify_domain_existance(
    conn: &Connection,
    domain: &String,
    rule: &str,
) -> Result<bool, SQLError> {
    let get_table = conn.query_row(
        format!(
            "SELECT * FROM {} WHERE main_domain='{}' AND rule='{}'",
            TABLE_NAME, domain, rule
        )
        .as_str(),
        [],
        |row| row.get::<usize, String>(0),
    );

    if get_table.is_ok() {
        Ok(true)
    } else {
        let err = get_table.unwrap_err();

        if err == SQLError::QueryReturnedNoRows {
            Ok(false)
        } else {
            Err(err)
        }
    }
}
fn verify_if_table_exists(conn: &Connection, table_name: &str) -> Result<bool, SQLError> {
    let get_table = conn.query_row(
        "SELECT * FROM sqlite_master
            WHERE type='table' AND name=?",
        [table_name],
        |row| row.get::<usize, String>(0),
    );

    if get_table.is_ok() {
        Ok(true)
    } else {
        let err = get_table.unwrap_err();

        if err == SQLError::QueryReturnedNoRows {
            Ok(false)
        } else {
            Err(err)
        }
    }
}
fn create_table(conn: &Connection, table: &str) -> Result<usize, SQLError> {
    conn.execute(
        format!(
            "CREATE TABLE {} (
                sub_domain TINYTEXT,
                main_domain TINYTEXT,
                rule TINYTEXT
            )",
            table
        )
        .as_str(),
        [],
    )
}
pub fn set_rule(action: &str, domain: &String) {
    let rule = "REJECT";
    let get_ips = Command::new("dig")
        .arg("+short")
        .arg(domain)
        .output()
        .unwrap();

    let ips = String::from_utf8(get_ips.stdout).expect("Failed to parse output");

    let conn = Connection::open(DB_PATH.as_str()).unwrap();
    // Do verifications
    {
        let table_exists = verify_if_table_exists(&conn, TABLE_NAME).unwrap();
        if !table_exists {
            create_table(&conn, TABLE_NAME).unwrap();
        }

        let rule_exists = verify_domain_existance(&conn, domain, rule).unwrap();
        if action == "add" {
            if rule_exists {
                panic!("This rule already exists!");
            }
        } else {
            if !rule_exists {
                panic!("You are trying to remove a rule that does not exist!");
            }
        }
    }

    let action_param = if action == "add" { "A" } else { "D" };
    for line in ips.lines() {
        let output = set_on_iptables(action_param, line, rule);

        if output.status.success() {
            let base_str = format!("{} iptables rule to: ", rule);

            if action == "add" {
                add_rule_to_db(&conn, line, domain, rule).unwrap();

                println!("Added {}{}", base_str, line);
            } else {
                println!("Removed {}{}", base_str, line);
            }
        } else {
            eprintln!("Failed to {} REJECT iptables rule to: {}", action, line);
            eprintln!(
                "SubCommand Error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
