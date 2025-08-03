# texcli

A simple Rust CLI tool to quickly generate LaTeX documents from pre‑made templates.  
Perfect for creating math homework, exams, reports, and other documents with minimal typing.

---

## Features

- 🖋 **Pre‑made templates** — Default Danish A‑level math homework/exam template included.
- 📅 **Automatic date & author** — Fills in current date and your real name automatically.
- 🛡 **Safe** — Won’t overwrite existing files; only opens file if it already exists.
- ⚡ **Fast** — Creates and opens your `.tex` file in microseconds.
- 🔤 **Safe filenames** — Spaces in the title are automatically replaced with underscores in the filename.
- 📂 **Organized** — Files are always saved in your **Documents/texcli** folder.

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/JegErGed/texcli.git
cd texcli
```

### 2. Build the binary
```bash
cargo build --release
```

### 3. Add to your PATH
```bash
cp target/release/texcli /usr/local/bin/
```

---

## Usage

```bash
texcli [TITLE] [DATE] [AUTHOR] [TEMPLATE]
texcli --help | -h
```

### Arguments
- **TITLE** — Title of the document *(default: "Opgave YYYY-MM-DD")*
- **DATE** — Document date *(default: today’s date)*
- **AUTHOR** — Author name *(default: your real name)*
- **TEMPLATE** — Template name *(default: `default`)*

---

### Examples

```bash
# Create a Danish A-level math homework file
texcli "Differentialregning opgave" "2025-08-03" "Johan" default
# → Creates ~/Documents/Differentialregning_opgave.tex

# Minimal usage (everything auto-filled)
texcli
# → Creates ~/Documents/Opgave_2025-08-03.tex
```

---

## Templates

**Default Template:** Danish A‑level math homework/exam style.

Future templates might include:
- **Essay/report** — academic paper style
- **Lab report** — for science classes
- **Presentation notes** — slides handouts
- **Minimal article** — barebones article template

---

## Requirements

You need LaTeX installed to compile `.tex` files.  
On macOS:
```bash
brew install --cask mactex
```

Make sure `latexmk` is in your `$PATH`.

---

## Example workflow

```bash
# Create a new math homework file
texcli "Differentialregning opgave" "2025-08-03" "Johan" default

# Edit it anytime with:
code ~/Documents/Differentialregning_opgave.tex
```

---

## License
MIT License — feel free to use, modify, and share.
