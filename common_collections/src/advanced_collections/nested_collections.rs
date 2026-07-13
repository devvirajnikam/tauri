// Scenario:
// Real app data often needs more than one collection level, such as departments
// to users or users to permissions.
//
// Thinking:
// Nested collections are powerful but should stay readable. Type aliases help
// explain intent when the shape grows.

use std::collections::{HashMap, HashSet};

type PermissionsByUser = HashMap<String, HashSet<String>>;

pub fn run() {
    println!("\n28. Nested collections");

    let mut permissions: PermissionsByUser = HashMap::new();

    permissions
        .entry(String::from("Asha"))
        .or_default()
        .insert(String::from("invoice:view"));

    permissions
        .entry(String::from("Asha"))
        .or_default()
        .insert(String::from("invoice:edit"));

    println!("Permissions by user: {:?}", permissions);
}
