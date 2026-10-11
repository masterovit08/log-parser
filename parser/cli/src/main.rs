use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};
use anyhow::{Result, Context};
use log_parser::{analyze, parser, LogType};
use colored::Colorize;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze{file, format} => {
            run_analysis(&file, format)?;
        }
    }

    Ok(())
}

fn run_analysis(path: &Path, format: InputType) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Could not open file {:?}", path.display()))?;
    let reader = BufReader::new(file);

    let parser = parser(format.into()).context("Could not parse file")?;
    let result = analyze(reader, parser.as_ref()).context("Could not analyze file")?;
    let stat = &result.result;

    //println!("{:#?}", result);

    println!("{} {}", "PARSING:".cyan().bold(), path.display());
    println!("{} {:?}", "FORMAT:".cyan().bold(), format);
    println!();

    println!("{} {}", "Total lines:".bold(), result.total_lines);
    println!("{} {}", "Parsed lines:".bold(), result.parsed_lines.to_string().green());

    if result.failed_lines > 0 {
        println!("{} {}", "Failed lines:".bold(), result.failed_lines.to_string().red());
    } else {
        println!("{} {}", "Failed lines:".bold(), result.failed_lines.to_string().green());
    }

    println!();

    if stat.errors > 0 {
        println!("{} {}", "Log errors:".bold(), stat.errors.to_string().red());
    } else {
        println!("{} {}", "Log errors:".bold(), stat.errors.to_string().green());
    }

    println!("HTTP 2xx: {}", stat.http_2xx.to_string().green());
    println!("HTTP 4xx: {}", stat.http_4xx.to_string().yellow());
    println!("HTTP 5xx: {}", stat.http_5xx.to_string().red());

    if stat.duration_count > 0 {
        let average = stat.total_duration_ms / stat.duration_count as f64;
        println!("{} {:.2}", "Average duration:".bold(), average);
    } else {
        println!("{} N/A", "Average duration:".bold());
    }

    if !result.errors.is_empty() {
        println!();
        println!("{}", "Parsing errors:".bold().underline());

        for error in &result.errors {
            println!("Line {}: {}", error.line, error.message.red());
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "log-parser")]
#[command(version)]
#[command(about = "CLI for log-parser")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Analyze a log file
    Analyze {
        /// Path to the log file
        file: PathBuf,

        /// Log format
        #[arg(long, short, value_enum)]
        format: InputType
    }
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum InputType {
    Jsonl,
    Nginx
}

impl From<InputType> for LogType {
    fn from(format: InputType) -> Self {
        match format {
            InputType::Jsonl => LogType::JsonLines,
            InputType::Nginx => LogType::NginxCombined,
        }
    }
}