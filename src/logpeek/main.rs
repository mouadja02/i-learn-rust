use std::fs;
use std::str;
use clap::Parser;

// Example usage:
// cargo run --bin logpeek -- logs/app.log --contains "timeout"
// logpeek help exploring the log files easily
// logpeek app.log --errors , shows the error lines in the log file 
// logpeek app.log --contains "timeout" , shows the lines that contains the word "timeout"
// logpeek app.log --last 50 , shows the last 50 lines of the log file
// logpeek app.log --json-summary , formats the log file as json and shows a summary of the log levels count

#[derive(Parser)]
#[command(name = "logpeek", about = "Inspect log files")]
struct Cli {
    /// Path to the log file
    file: String,

    /// Show only ERROR lines
    #[arg(long)]
    errors: bool,

    
    /// Show only DEBUG lines
    #[arg(long)]
    debug: bool,


    /// Show lines containing this string
    #[arg(long)]
    contains: Option<String>,

    /// Show last N lines
    #[arg(long)]
    last: Option<usize>,

    /// Output a JSON summary
    #[arg(long)]
    json_summary: bool,
}

#[derive(Clone)]
struct Line {
    date: String,
    time: String,
    level: String,
    component: String,
    message: String,
}

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let data: Vec<u8> = fs::read(path)?;
    Ok(data)
}

fn structuring_raw_logs(path: &str) -> Vec<Line> {
    let mut log_lines: Vec<Line> = Vec::new();
    match read_file(path) {
        Ok(data) => {
            println!("File read successfully. Size: {} bytes", data.len());
            match str::from_utf8(&data){
                Ok(text) => {
                    let log_content: Vec<&str> = text.split('\n').collect();
                    for part in log_content {
                        let level = part.split(" ").nth(2).unwrap_or("");
                        if level.len() == 4 {
                            log_lines.push(Line {
                                date: part.split("  ").nth(0).unwrap_or("").to_string().split(" ").nth(0).unwrap_or("").to_string(),
                                time: part.split("  ").nth(0).unwrap_or("").to_string().split(" ").nth(1).unwrap_or("").to_string(),
                                level: part.split("  ").nth(0).unwrap_or("").to_string().split(" ").nth(2).unwrap_or("").to_string(),
                                component: part.split("] ").nth(0).unwrap_or("").to_string().split("[").nth(1).unwrap_or("").to_string(),
                                message: part.split("] ").nth(1).unwrap_or("").to_string(),
                        });
                        }
                        else if level.len() == 5 {
                           log_lines.push(Line {
                                date: part.split(" ").nth(0).unwrap_or("").to_string(),
                                time: part.split(" ").nth(1).unwrap_or("").to_string(),
                                level: part.split(" ").nth(2).unwrap_or("").to_string(),
                                component: part.split("] ").nth(0).unwrap_or("").to_string().split("[").nth(1).unwrap_or("").to_string(),
                                message: part.split("] ").nth(1).unwrap_or("").to_string(),
                        }); 
                        }
                    }
                },
                Err(_) => println!("File content is not valid UTF-8."),
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    log_lines
}


fn print_line_table(log_lines: &Vec<Line>) {
    println!("{:<10} | {:<10} | {:<10} | {:<20} | {}", "Date", "Time", "Level", "Component", "Message");
    println!("{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<20}-+-{:-<50}", "", "", "", "", "");
    for log_line in log_lines {
        println!("{:<10} | {:<10} | {:<10} | {:<20} | {}", log_line.date, log_line.time, log_line.level, log_line.component, log_line.message);
    }
}

fn filter_lines_by_level(log_lines: &Vec<Line>, level: &str) -> Vec<Line> {
    let mut filtered_lines: Vec<Line> = Vec::new();
    for line in log_lines {
        if line.level == level {
            filtered_lines.push(line.clone());
        }
    }
    filtered_lines
}

fn filter_lines_by_keyword(log_lines: &Vec<Line>, keyword: &str) -> Vec<Line> {
    let mut filtered_lines: Vec<Line> = Vec::new();
    for line in log_lines.iter() {
        if line.message.contains(keyword) {
            filtered_lines.push(line.clone());
        }
    }
    filtered_lines
}

fn get_n_lines(log_lines: &Vec<Line>, n: usize, start_or_end: &str) -> Vec<Line> {
    let total_lines = log_lines.len();
    if n >= total_lines {
        return log_lines.clone();
    }
    if start_or_end == "start" {
        return log_lines[0..n].to_vec();
    } else {
        return log_lines[total_lines - n..total_lines].to_vec();
    }
}

fn export_as_json(log_lines: &Vec<Line>) -> String {
    let mut json_output = String::from("[\n");
    for line in log_lines {
        json_output.push_str("{\n");
        json_output.push_str(&format!("  \"date\": \"{}\",\n", line.date));
        json_output.push_str(&format!("  \"time\": \"{}\",\n", line.time));
        json_output.push_str(&format!("  \"level\": \"{}\",\n", line.level));
        json_output.push_str(&format!("  \"component\": \"{}\",\n", line.component));
        json_output.push_str(&format!("  \"message\": \"{}\"\n", line.message));
        json_output.push_str("},\n");
    }
    if !log_lines.is_empty() {
        json_output.truncate(json_output.len() - 2); // Remove the last comma and newline
    }
    json_output.push_str("\n]");
    json_output
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();
    let file_path = &cli.file;
    let mut log_lines: Vec<Line> = structuring_raw_logs(file_path);
    if cli.errors {
        log_lines = filter_lines_by_level(&log_lines, "ERROR");
    }
    if cli.debug {
        log_lines = filter_lines_by_level(&log_lines, "DEBUG");
    }
    if let Some(keyword) = cli.contains {
        log_lines = filter_lines_by_keyword(&log_lines, &keyword);
    }
    if let Some(n) = cli.last {
        log_lines = get_n_lines(&log_lines, n, "end");
    }
    if cli.json_summary {
        let json_output = export_as_json(&log_lines);
        println!("{}", json_output);
    } else {
        print_line_table(&log_lines);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let path = "../../test_data/sample.txt";
        let result = read_file(path);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_read_file_not_found() {
        let path = "non_existent_file.txt";
        let result = read_file(path);
        assert!(result.is_err());
    }
}
