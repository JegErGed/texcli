use chrono::Local;
use fstrings::*;
use std::{env, fs, path::PathBuf, process::{self, Command}};
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
    let default: String = f!(r#"\documentclass[11pt,a4paper]{{article}}
\usepackage[utf8]{{inputenc}}
\usepackage[T1]{{fontenc}}
\usepackage[danish]{{babel}}
\usepackage{{amsmath, amssymb, amsfonts}}
\usepackage{{geometry}}
\geometry{{ top=2.5cm, bottom=2.5cm, left=3cm, right=3cm }}
\usepackage{{fancyhdr}}
\usepackage{{lastpage}}
\pagestyle{{fancy}}
\fancyhf{{}}
\fancyhead[L]{{\textbf{{Matematik A - Opgave}}}}
\fancyhead[R]{{\thepage\ af \pageref{{LastPage}}}}
\usepackage{{graphicx}}
\graphicspath{{{{figure/}}}}
\usepackage{{enumitem}}
\usepackage{{hyperref}}

\title{{\textbf{{{titel}}}}}
\author{{Navn: {author}}}
\date{{Dato: {date}}}

\begin{{document}}

\maketitle
\thispagestyle{{fancy}}

\section*{{Opgave 1}}
\subsection*{{(a)}} Tekst og udregninger her.

\subsection*{{(b)}} Eksempel på graf:
\begin{{figure}}[h]
    \centering
    \includegraphics[width=0.8\textwidth]{{sample-graph.pdf}}
    \caption{{Graf over funktion $f(x)$.}}
\end{{figure}}

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

fn write_sample_graph(latex_dir: &PathBuf) -> std::io::Result<()> {
    let bytes = include_bytes!("assets/sample-graph.pdf"); // Embed file at compile time
    let fig_dir = latex_dir.join("figure");

    fs::create_dir_all(&fig_dir)?;
    let file_path = fig_dir.join("sample-graph.pdf");

    if !file_path.exists() {
        fs::write(&file_path, bytes)?;
        println!("Created sample-graph.pdf in {:?}", fig_dir);
    }
    Ok(())
}

fn main() {
    let cur_date = Local::now().format("%Y-%m-%d").to_string();
    let username = whoami::realname();

    // Parse CLI args
    let args: Vec<String> = env::args().collect();
    let titel = args.get(1).cloned().unwrap_or(f!("Opgave {cur_date}"));
    let date = args.get(2).cloned().unwrap_or(cur_date.clone());
    let author = args.get(3).cloned().unwrap_or(username.clone());
    let template_name = args
        .get(4)
        .cloned()
        .unwrap_or_else(|| "default".to_string());

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        process::exit(0);
    }

    // Print debug info
    println!("Title: {}", titel);
    println!("Date: {}", date);
    println!("Author: {}", author);
    println!("Template name: {}", template_name);

    // Get Documents directory path for the current user
    let documents_dir = dirs::document_dir().expect("Could not find Documents directory");

    // Create a "texcli" subfolder in Documents
    let mut texcli_dir = documents_dir.clone();
    texcli_dir.push("texcli");
    fs::create_dir_all(&texcli_dir).expect("Failed to create texcli directory");

    // Make sure the filename is safe (replace spaces with underscores)
    let safe_title = titel.replace(' ', "_");

    let mut root_path = PathBuf::from(&documents_dir);
    root_path.push("texcli");
    root_path.push(&safe_title);

    let latex_dir = root_path.join("latex");
    let figure_dir = latex_dir.join("figure");
    let notebook_dir = root_path.join("notebook");
    write_sample_graph(&latex_dir).unwrap();

    // Create directories
    fs::create_dir_all(&figure_dir).expect("Failed to create figure directory");
    fs::create_dir_all(&notebook_dir).expect("Failed to create notebook directory");

    // LaTeX file path
    let tex_file = latex_dir.join("main.tex");

    if tex_file.exists() {
        println!("Error: '{}' already exists. Aborting.", tex_file.display());
    } else {
        fs::write(
            &tex_file,
            get_template(&template_name, &titel, &author, &date),
        )
        .expect("Failed to write LaTeX file");
    }
    // Create empty Jupyter notebook
    let notebook_content = r#"{
 "cells": [],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 5
}"#;
    fs::write(notebook_dir.join("work.ipynb"), notebook_content)
        .expect("Failed to create Jupyter notebook");

    println!("Created assignment folder at: {}", root_path.display());

    // Try to open the file in VSCode
    let status = Command::new("code").arg(root_path).status();

    match status {
        Ok(status) if status.success() => println!("Opened file in VSCode."),
        Ok(status) => println!("VSCode exited with status: {}", status),
        Err(e) => println!("Failed to open VSCode: {}", e),
    }
}
