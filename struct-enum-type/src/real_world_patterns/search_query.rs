// Scenario:
// A listing page needs filters, sorting, and pagination for data like users,
// invoices, or items.
//
// Thinking:
// Use a struct for the full query because all fields travel together. Use enums
// for sort fields and direction so invalid sort strings are not accepted inside
// the core code.

#[derive(Debug)]
enum SortField {
    Name,
    CreatedAt,
}

#[derive(Debug)]
enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug)]
struct SearchQuery {
    text: String,
    page: u32,
    per_page: u32,
    sort_field: SortField,
    sort_direction: SortDirection,
}

pub fn run() {
    println!("\n42. Search query structs");

    let query = SearchQuery {
        text: String::from("keyboard"),
        page: 1,
        per_page: 20,
        sort_field: SortField::CreatedAt,
        sort_direction: SortDirection::Desc,
    };

    println!("{:?}", query);
    println!(
        "Query fields -> text: {}, page: {}, per_page: {}, sort: {:?} {:?}",
        query.text, query.page, query.per_page, query.sort_field, query.sort_direction
    );

    let name_ascending = SearchQuery {
        text: String::from("mouse"),
        page: 2,
        per_page: 10,
        sort_field: SortField::Name,
        sort_direction: SortDirection::Asc,
    };

    println!("Another query: {:?}", name_ascending);
}
