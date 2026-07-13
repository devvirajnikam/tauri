// Scenario:
// An application needs to decide whether a user can view, edit, delete, or
// manage records.
//
// Thinking:
// Use enums for roles and permissions because they are closed sets of known
// values. Then keep permission logic in one function instead of scattering it.

#[derive(Debug, PartialEq)]
enum Role {
    Admin,
    Manager,
    Viewer,
}

#[derive(Debug, PartialEq)]
enum Permission {
    View,
    Edit,
    Delete,
}

fn has_permission(role: &Role, permission: &Permission) -> bool {
    match role {
        Role::Admin => true,
        Role::Manager => matches!(permission, Permission::View | Permission::Edit),
        Role::Viewer => matches!(permission, Permission::View),
    }
}

pub fn run() {
    println!("\n35. Role and permission enums");

    let roles = [Role::Admin, Role::Manager, Role::Viewer];
    let permissions = [Permission::View, Permission::Edit, Permission::Delete];

    for role in roles {
        for permission in &permissions {
            println!(
                "{:?} can {:?}: {}",
                role,
                permission,
                has_permission(&role, permission)
            );
        }
    }
}
