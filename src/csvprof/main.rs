use std::fs;
use std::str;
use clap::Parser;

// Example usage:
// cargo run --bin csvprof data.csv --columns "name,age" --filter "age>30" --sort "age" --output "filtered_data.csv"
// csvprof help exploring the csv files easily
// csvprof data.csv --columns "name,age" , shows only the name and age
// csvprof data.csv --filter "age>30" , shows only the rows where age
// csvprof data.csv --sort "age" , sorts the rows by age
// csvprof data.csv --output "filtered_data.csv" , saves the filtered data to a


#[derive(Parser)]
#[command(name = "csvprof", about = "Profile CSV files")]
struct Cli {
    /// Path to the CSV file
    file: String,

    /// Comma-separated list of columns to display
    #[arg(long)]
    columns: bool,

    /// Filter expression (e.g., "age>30")
    #[arg(long)]
    filter: Option<String>,

    /// Column name to sort by (e.g., "age") + optional sort order (asc/desc) (e.g., "age:desc", "name:asc")
    #[arg(long)]
    sort: Option<String>,


    /// Number of rows to display (default is all)
    /// If specified, only the top N rows after filtering and sorting will be shown
    #[arg(long)]
    limit: Option<usize>,

    // Schema display flag
    #[arg(long)]
    schema: bool,

    /// Output file for filtered data
    #[arg(long)]
    output: Option<String>, // .json or .csv or .tsv
}

#[derive(Clone)]
struct CSV {
    header: Vec<String>,
    columns_count: usize,
    rows_count: usize,
    rows: Vec<Vec<String>>,
    max_columns_width: Vec<usize>,

}

#[derive(Clone)]
struct ColumnDetails {
    name: String,
    type_guess: String,
    count: usize, // number of non-null values
    nulls_count: usize,
    max_value: String,
    min_value: String,
    distinct_values_count: usize,
    most_common_value: String,
    most_common_value_count: usize,
}

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let data: Vec<u8> = fs::read(path)?;
    Ok(data)
}

fn structuring_csv_file(path: &str) -> CSV {
    let data = match read_file(path) {
    Ok(d) => d,
    Err(e) => {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
        }
    };
    println!("File read successfully. Size: {} bytes", data.len());
    let text = match str::from_utf8(&data) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("File content is not valid UTF-8.");
            std::process::exit(1);
        }
    };
    // Strip UTF-8 BOM added by Windows tools (e.g. PowerShell Out-File)
    let text = text.strip_prefix('\u{FEFF}').unwrap_or(text);
    // .lines() handles both \r\n and \n endings correctly
    let csv_content: Vec<&str> = text.lines().collect();
    let header: Vec<String> = csv_content[0].split(',').map(str::to_string).collect();
    let columns_count = header.len();
    let rows: Vec<Vec<String>> = csv_content[1..].iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').map(str::to_string).collect())
        .collect();
    let rows_count = rows.len();
    let max_columns_width: Vec<usize> = (0..columns_count).map(|i| {
        let max_width_in_column = rows.iter()
            .map(|row| row.get(i).map_or(0, |s| s.len()))
            .max().unwrap_or(0);
        let header_width = header[i].len();
        max_width_in_column.max(header_width)
    }).collect();
    CSV { header, columns_count, rows_count, rows, max_columns_width }
}

fn print_line_table(csv: &CSV) {
    // Print the header
    let mut line1: String = "".to_string();
    let mut line2: String = "".to_string();
    let mut line3: String = "".to_string();
    for (i, column) in csv.header.iter().enumerate() {
        if i == 0 {
            line1 += &format!("+-{:-<width$}-+", "", width = csv.max_columns_width[i]);
            line2 += &format!("| {:<width$} |", column, width = csv.max_columns_width[i]);
            line3 += &format!("+-{:-<width$}-+", "", width = csv.max_columns_width[i]);
        } else {
        line1 += &format!("{:-<width$}-+", "", width = csv.max_columns_width[i]);
        line2 += &format!("{:<width$} |", column, width = csv.max_columns_width[i]);
        line3 += &format!("{:-<width$}-+", "", width = csv.max_columns_width[i]);
    }
    }
    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);
    for row in &csv.rows {
        let mut line: String = "".to_string();
        for i in 0..csv.columns_count {
            let val = row.get(i).map(String::as_str).unwrap_or("");
            if i == 0 {
                line += &format!("| {:<width$} |", val, width = csv.max_columns_width[i]);
            } else {
                line += &format!("{:<width$} |", val, width = csv.max_columns_width[i]);
            }
        }
        println!("{}", line);
    }
    for i in 0..csv.columns_count {
        if i == 0 {
            print!("+-{:-<width$}-+", "", width = csv.max_columns_width[i]);
        } else {
            print!("{:-<width$}-+", "", width = csv.max_columns_width[i]);
        }
    }
    println!();
}

fn type_guessing(column_values: &Vec<String>) -> String {
    let mut is_integer = true;
    let mut is_float = true;
    for value in column_values.iter().filter(|v| !v.is_empty()) {
        if value.parse::<i64>().is_err() {
            is_integer = false;
        }
        if value.parse::<f64>().is_err() {
            is_float = false;
        }
    }
    if is_integer {
        "Integer".to_string()
    } else if is_float {
        "Float".to_string()
    } else {
        "String".to_string()
    }
}

fn fetch_columns_details(csv: &CSV) -> Vec<ColumnDetails> {
    let mut column_details: Vec<ColumnDetails> = Vec::new();
    for i in 0..csv.columns_count {
        let column_values: Vec<String> = csv.rows.iter().map(|row| row.get(i).cloned().unwrap_or("".to_string())).collect();
        let name = csv.header[i].clone();
        let type_guess = type_guessing(&column_values);
        let count = column_values.iter().filter(|v| !v.is_empty()).count();
        let nulls_count = column_values.iter().filter(|v| v.is_empty()).count();
        let non_null_values: Vec<&String> = column_values.iter().filter(|v| !v.is_empty()).collect();
        let max_value = non_null_values.iter().max().map(|v| v.to_string()).unwrap_or("".to_string());
        let min_value = non_null_values.iter().min().map(|v| v.to_string()).unwrap_or("".to_string());
        let distinct_values_count = non_null_values.iter().collect::<std::collections::HashSet<_>>().len();
        let most_common_value = non_null_values.iter()
            .fold(std::collections::HashMap::new(), |mut acc, v| {
                *acc.entry(*v).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(value, _)| value.clone())
            .unwrap_or("".to_string());
        let most_common_value_count = non_null_values.iter().filter(|v| ***v == most_common_value).count();
        column_details.push(ColumnDetails {
            name,
            type_guess,
            count,
            nulls_count,
            max_value,
            min_value,
            distinct_values_count,
            most_common_value,
            most_common_value_count,
        });
    }
    column_details
}

fn sort_by_column(csv: &mut CSV, column_name: &str, descending: bool) {
    let col_idx = match csv.header.iter().position(|h| h == column_name) {
        Some(idx) => idx,
        None => {
            eprintln!("Column '{}' not found in CSV header.", column_name);
            std::process::exit(1);
        }
    };
    let col_values: Vec<String> = csv.rows.iter()
        .map(|row| row.get(col_idx).cloned().unwrap_or_default())
        .collect();
    let type_guess = type_guessing(&col_values);
    csv.rows.sort_by(|a, b| {
        let va = a.get(col_idx).map(String::as_str).unwrap_or("");
        let vb = b.get(col_idx).map(String::as_str).unwrap_or("");
        let ord = match type_guess.as_str() {
            "Integer" => {
                let ia = va.parse::<i64>().unwrap_or(i64::MIN);
                let ib = vb.parse::<i64>().unwrap_or(i64::MIN);
                ia.cmp(&ib)
            }
            "Float" => {
                let fa = va.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                let fb = vb.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                fa.partial_cmp(&fb).unwrap_or(std::cmp::Ordering::Equal)
            }
            _ => va.cmp(vb),
        };
        if descending { ord.reverse() } else { ord }
    });
}

fn filter_by_column_value(csv: &CSV, column_name: &str, operator: &str, value: &str) -> Vec<Vec<String>> {
    let col_idx = match csv.header.iter().position(|h| h == column_name) {
        Some(idx) => idx,
        None => {
            eprintln!("Column '{}' not found in CSV header.", column_name);
            std::process::exit(1);
        }
    };
    let col_values: Vec<String> = csv.rows.iter()
        .map(|row| row.get(col_idx).cloned().unwrap_or_default())
        .collect();
    let type_guess = type_guessing(&col_values);
    csv.rows.iter().filter(|row| {
        let cell_value = row.get(col_idx).map(String::as_str).unwrap_or("");
        match type_guess.as_str() {
            "Integer" => {
                let cell_int = cell_value.parse::<i64>().unwrap_or(i64::MIN);
                let filter_int = value.parse::<i64>().unwrap_or(i64::MIN);
                match operator {
                    ">" => cell_int > filter_int,
                    "<" => cell_int < filter_int,
                    ">=" => cell_int >= filter_int,
                    "<=" => cell_int <= filter_int,
                    "==" => cell_int == filter_int,
                    "!=" => cell_int != filter_int,
                    _ => false,
                }
            }
            "Float" => {
                let cell_float = cell_value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                let filter_float = value.parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                match operator {
                    ">" => cell_float > filter_float,
                    "<" => cell_float < filter_float,
                    ">=" => cell_float >= filter_float,
                    "<=" => cell_float <= filter_float,
                    "==" => cell_float == filter_float,
                    "!=" => cell_float != filter_float,
                    _ => false,
                }
            }
            _ => {
                match operator {
                    "==" => cell_value == value,
                    "!=" => cell_value != value,
                    _ => false,
                }
            }
        }
    }).cloned().collect()
}

fn save_filtered_data(csv: &CSV, output_path: &str) {
    if !output_path.ends_with(".csv") && !output_path.ends_with(".json") && !output_path.ends_with(".tsv") {
        eprintln!("Unsupported output format. Please use .csv, .json, or .tsv");
        std::process::exit(1);
    }
    if output_path.ends_with(".csv") {
        let mut output = String::new();
        output += &(csv.header.join(",") + "\n");
        for row in &csv.rows {
            output += &(row.join(",") + "\n");
        }
        match fs::write(output_path, output) {
            Ok(_) => println!("Filtered data saved to '{}'", output_path),
            Err(e) => eprintln!("Error saving filtered data: {}", e),
        }
    } else if output_path.ends_with(".json") {
        let mut output = String::new();
        output += "[\n\t{\n";
        for row in &csv.rows {
            for (i, column) in csv.header.iter().enumerate() {
                let value = row.get(i).map(String::as_str).unwrap_or("");
                output += &format!("\t\t\"{}\": \"{}\"", column, value);
                if i < csv.columns_count - 1 {
                    output += ",\n";
                } else {
                    output += "\n";
                }
            }
            output += "\t}";
            if row != csv.rows.last().unwrap() {
                output += ",\n\t{\n";
            } else {
                output += "\n";
            }
        }
        output += "\n]";
        match fs::write(output_path, output) {
            Ok(_) => println!("Filtered data saved to '{}'", output_path),
            Err(e) => eprintln!("Error saving filtered data: {}", e),
        }
    } else if output_path.ends_with(".tsv") {
        let mut output = String::new();
        output += &csv.header.join("\t");
        output += "\n";
        for row in &csv.rows {
            output += &row.join("\t");
            output += "\n";
        }
        match fs::write(output_path, output) {
            Ok(_) => println!("Filtered data saved to '{}'", output_path),
            Err(e) => eprintln!("Error saving filtered data: {}", e),
        }
    }
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();
    let file_path = &cli.file;
    let mut csv_file: CSV = structuring_csv_file(file_path);

    if let Some(sort_arg) = &cli.sort {
        let parts: Vec<&str> = sort_arg.splitn(2, ':').collect();
        let col_name = parts[0].trim();
        let descending = parts.get(1).map(|s| s.trim().eq_ignore_ascii_case("desc")).unwrap_or(false);
        sort_by_column(&mut csv_file, col_name, descending);
    }

    if let Some(filter_arg) = &cli.filter {
        let operators = [">=", "<=", ">", "<", "==", "!="];
        let mut operator_found = None;
        for op in &operators {
            if filter_arg.contains(op) {
                operator_found = Some(*op);
                break;
            }
        }
        let operator = match operator_found {
            Some(op) => op,
            None => {
                eprintln!("Invalid filter expression. Supported operators: >, <, >=, <=, ==, !=");
                std::process::exit(1);
            }
        };
        let parts: Vec<&str> = filter_arg.splitn(2, operator).collect();
        if parts.len() != 2 {
            eprintln!("Invalid filter expression format. Expected format: column_name operator value (e.g., age>30)");
            std::process::exit(1);
        }
        let column_name = parts[0].trim();
        let value = parts[1].trim();
        csv_file.rows = filter_by_column_value(&csv_file, column_name, operator, value);
        csv_file.rows_count = csv_file.rows.len();
    }
    
    if let Some(limit) = cli.limit {
        csv_file.rows.truncate(limit);
        csv_file.rows_count = csv_file.rows.len();
    }

    if cli.columns {
        let column_details = fetch_columns_details(&csv_file);
        // display column details as table
        println!("Column Details:");
        println!("{:<10} | {:<10} | {:<10} | {:<10} | {:<20} | {:<20} | {:<20} | {:<20} | {:<20}", "Name", "Type", "Count", "Nulls", "Max Value", "Min Value", "Distinct Values", "Most Common Value", "Most Common Value Count");
        println!("{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<20}-+-{:-<20}-+-{:-<20}-+-{:-<20}-+-{:-<20}", "", "", "", "", "", "", "", "", "");
        for detail in column_details {
            println!("{:<10} | {:<10} | {:<10} | {:<10} | {:<20} | {:<20} | {:<20} | {:<20} | {:<20}", detail.name, detail.type_guess, detail.count, detail.nulls_count, detail.max_value, detail.min_value, detail.distinct_values_count, detail.most_common_value, detail.most_common_value_count);
        }
    }
    else if cli.schema {
        println!("Schema:");
        for i in 0..csv_file.columns_count {
            println!("{}: {}", csv_file.header[i], type_guessing(&csv_file.rows.iter().map(|row| row.get(i).cloned().unwrap_or_default()).collect()));
        }
    }
    else {
        print_line_table(&csv_file);
    }

    if let Some(output_path) = &cli.output {
        save_filtered_data(&csv_file, output_path);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        file
    }

    #[test]
    fn test_read_file_success() {
        let file = create_temp_csv("name,age\nAlice,30\nBob,25\n");
        let result = read_file(file.path().to_str().unwrap());
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read_file("non_existent_file.csv");
        assert!(result.is_err());
    }

    #[test]
    fn test_structuring_csv_header() {
        let file = create_temp_csv("name,age,city\nAlice,30,Paris\nBob,25,Lyon\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.header, vec!["name", "age", "city"]);
    }

    #[test]
    fn test_structuring_csv_columns_count() {
        let file = create_temp_csv("name,age,city\nAlice,30,Paris\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.columns_count, 3);
    }

    #[test]
    fn test_structuring_csv_rows_count() {
        let file = create_temp_csv("name,age\nAlice,30\nBob,25\nCharlie,35\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.rows_count, 3);
    }

    #[test]
    fn test_structuring_csv_rows_content() {
        let file = create_temp_csv("name,age\nAlice,30\nBob,25\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.rows[0], vec!["Alice", "30"]);
        assert_eq!(csv.rows[1], vec!["Bob", "25"]);
    }

    #[test]
    fn test_structuring_csv_empty_lines_filtered() {
        let file = create_temp_csv("name,age\nAlice,30\n\nBob,25\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.rows_count, 2);
    }

    #[test]
    fn test_structuring_csv_single_row() {
        let file = create_temp_csv("id,value\n42,hello\n");
        let csv = structuring_csv_file(file.path().to_str().unwrap());
        assert_eq!(csv.rows_count, 1);
        assert_eq!(csv.rows[0], vec!["42", "hello"]);
    }
}