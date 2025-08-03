use chrono::Local;
use fstrings::*;
use std::{env, fs, path::PathBuf, process::Command, process};
use whoami;

/// Show usage/help message
fn print_help() {
    println!(
        r#"texcli - Quick LaTeX file generator

Usage:
  texcli [TITLE] [DATE] [AUTHOR] [TEMPLATE]
  texcli --help | -h

Arguments:
  TITLE     Title of the document (default: "Opgave YYYY-MM-DD")
  DATE      Document date (default: today's date)
  AUTHOR    Author name (default: your real name)
  TEMPLATE  Template name (default: "default")

Examples:
  texcli "Differentialregning opgave" "2025-08-03" "Johan" default
  texcli
  texcli --help

Notes:
  - Spaces in TITLE are replaced with dashes in the filename.
  - Program aborts if the file already exists.
"#
    );
}

/// Get LaTeX template and fill in variables
fn get_template(template_name: &str, titel: &str, author: &str, date: &str) -> String {
    // Danish A-level math homework/exam template as default
    let default: String = f!(r#"
\documentclass[11pt,a4paper]{{article}}

\usepackage[utf8]{{inputenc}}
\usepackage[T1]{{fontenc}}
\usepackage[danish]{{babel}}
\usepackage{{amsmath, amssymb, amsfonts}}
\usepackage{{geometry}}
\usepackage{{fancyhdr}}
\usepackage{{lastpage}}
\usepackage{{graphicx}}
\usepackage{{enumitem}}
\usepackage{{hyperref}}

% Margins
\geometry{{
  top=2.5cm,
  bottom=2.5cm,
  left=3cm,
  right=3cm
}}

% Header and footer
\pagestyle{{fancy}}
\fancyhf{{}}
\fancyhead[L]{{\textbf{{Matematik A - Opgave}}}}
\fancyhead[C]{{}}
\fancyhead[R]{{\thepage\ af \pageref{{LastPage}}}}
\fancyfoot[C]{{}}

% Document info (to be replaced by your code)
\title{{\textbf{{{titel}}}}}
\author{{Navn: {author}}}
\date{{Dato: {date}}}

\begin{{document}}

\maketitle
\thispagestyle{{fancy}}

\section*{{Opgave 1}}
% Skriv opgave 1 her
\begin{{enumerate}}[label=(\alph*)]
  \item 
  \item 
  \item 
\end{{enumerate}}

\section*{{Opgave 2}}
% Skriv opgave 2 her
\begin{{enumerate}}[label=(\alph*)]
  \item 
  \item 
  \item 
\end{{enumerate}}

% Tilføj flere opgaver efter behov

\end{{document}}
"#);

    match template_name {
        "default" => default,
        _ => {
            println!("Template unrecognised, will use default");
            default
        }
    }
}

fn main() {
    let cur_date = Local::now().format("%Y-%m-%d").to_string();
    let username = whoami::realname();

    // Parse CLI args
    let args: Vec<String> = env::args().collect();
    let titel = args.get(1).cloned().unwrap_or(f!("Opgave {cur_date}"));
    let date = args.get(2).cloned().unwrap_or(cur_date.clone());
    let author = args.get(3).cloned().unwrap_or(username.clone());
    let template_name = args.get(4).cloned().unwrap_or_else(|| "default".to_string());

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        process::exit(0);
    }

    // Get template content with variables substituted
    let template_content = get_template(&template_name, &titel, &author, &date);

    // Print debug info
    println!("Title: {}", titel);
    println!("Date: {}", date);
    println!("Author: {}", author);
    println!("Template name: {}", template_name);

    // Get Documents directory path for the current user
    let documents_dir = dirs::document_dir().expect("Could not find Documents directory");

    // Make sure the filename is safe (replace spaces with underscores)
    let safe_title = titel.replace(' ', "_");

    // Build the full file path
    let mut filepath = PathBuf::from(&documents_dir);
    filepath.push(format!("{safe_title}.tex"));

    // Abort if file already exists.
    if filepath.exists() {
        eprintln!("Error: File '{}' already exists. Aborting to avoid overwrite.", filepath.display());
        std::process::exit(1);
    }

    // Write the LaTeX content to the file
    fs::write(&filepath, template_content).expect("Failed to write LaTeX file");

    println!("Written LaTeX file to: {}", filepath.display());

    // Try to open the file in VSCode
    let status = Command::new("code")
        .arg(filepath)
        .status();

    match status {
        Ok(status) if status.success() => println!("Opened file in VSCode."),
        Ok(status) => println!("VSCode exited with status: {}", status),
        Err(e) => println!("Failed to open VSCode: {}", e),
    }
}
