// src/output.rs
use anyhow::Result;
use clap::ValueEnum;
use serde_json::Value;

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Json,
    Table,
}

pub fn print_output(value: &Value, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(value)?),
        OutputFormat::Table => print_table(value),
    }
    Ok(())
}

fn print_table(value: &Value) {
    match value {
        Value::Array(items) => print_array_table(items),
        Value::Object(_) => print_object_table(value),
        other => println!("{other}"),
    }
}

fn print_array_table(items: &[Value]) {
    if items.is_empty() {
        println!("(empty)");
        return;
    }
    let headers: Vec<String> = match &items[0] {
        Value::Object(map) => map.keys().cloned().collect(),
        _ => {
            for item in items {
                println!("{}", value_to_cell(item));
            }
            return;
        }
    };
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|item| {
            headers
                .iter()
                .map(|h| value_to_cell(item.get(h).unwrap_or(&Value::Null)))
                .collect()
        })
        .collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    let header_row: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{header_row}");
    let sep: String = widths.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("  ");
    println!("{sep}");
    for row in &rows {
        let line: String = row
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{:<width$}", cell, width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ");
        println!("{line}");
    }
}

fn print_object_table(value: &Value) {
    let Value::Object(map) = value else { return };
    let key_width = map.keys().map(|k| k.len()).max().unwrap_or(0);
    for (k, v) in map {
        println!("{:<width$}  {}", k, value_to_cell(v), width = key_width);
    }
}

fn value_to_cell(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
