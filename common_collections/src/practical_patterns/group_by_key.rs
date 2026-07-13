// Scenario:
// You have many records and need to group them by a field, such as department.
//
// Thinking:
// HashMap<Key, Vec<Value>> is the standard group-by shape.

use std::collections::HashMap;

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
}

pub fn run() {
    println!("\n12. Group by key");

    let employees = vec![
        Employee {
            name: String::from("Asha"),
            department: String::from("Engineering"),
        },
        Employee {
            name: String::from("Ravi"),
            department: String::from("Sales"),
        },
        Employee {
            name: String::from("Nina"),
            department: String::from("Engineering"),
        },
    ];

    let mut by_department: HashMap<String, Vec<String>> = HashMap::new();

    for employee in employees {
        by_department
            .entry(employee.department)
            .or_default()
            .push(employee.name);
    }

    println!("Employees by department: {:?}", by_department);
}
