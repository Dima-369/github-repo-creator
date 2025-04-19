# GitHub Repository Creator

A command-line tool written in Rust that allows you to quickly create GitHub repositories with an interactive interface.

## Features

- Interactive command-line interface with proper cursor movement and editing
- Create public or private GitHub repositories
- Add optional repository descriptions
- Automatically initializes repositories with a README
- Simple and intuitive user experience

## Installation

### Prerequisites

- Rust and Cargo installed on your system
- A GitHub personal access token with appropriate permissions

### Building from source

```bash
git clone https://github.com/yourusername/github-repo-creator.git
cd github-repo-creator
cargo build --release
```

The compiled binary will be available at `target/release/github-repo-creater`.

## Usage

```bash
github-repo-creater --token YOUR_GITHUB_TOKEN
```

The tool will guide you through the repository creation process with interactive prompts:

1. Enter repository name (required)
2. Enter repository description (optional)
3. Choose whether the repository should be private or public

After completing these steps, the tool will create the repository on GitHub and provide you with the repository URL and git clone command.