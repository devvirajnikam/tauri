// Scenario:
// The same data should be formatted in different ways.
//
// Thinking:
// A formatter trait separates data from presentation.

struct Report {
    title: String,
    count: u32,
}

trait ReportFormatter {
    fn format(&self, report: &Report) -> String;
}

struct PlainTextFormatter;

impl ReportFormatter for PlainTextFormatter {
    fn format(&self, report: &Report) -> String {
        format!("{}: {}", report.title, report.count)
    }
}

struct JsonLikeFormatter;

impl ReportFormatter for JsonLikeFormatter {
    fn format(&self, report: &Report) -> String {
        format!(
            "{{ \"title\": \"{}\", \"count\": {} }}",
            report.title, report.count
        )
    }
}

pub fn run() {
    println!("\n9. Formatter trait");

    let report = Report {
        title: String::from("Users"),
        count: 3,
    };

    println!("{}", PlainTextFormatter.format(&report));
    println!("{}", JsonLikeFormatter.format(&report));
}
