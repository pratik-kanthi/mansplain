# mansplain

mansplain is a Command-Line Interface (CLI) tool built in Rust that mansplains a command to you

## Prerequisites

- Rust and Cargo installed
- An OpenAI API key

## Configuration

### 1. Clone the Repository

```bash
git clone https://github.com/pratik-kanthi/mansplain.git
cd mansplain
```

### 2. Environment Variable
Set the OpenAI API key as an environment variable. You can either do this manually before running the program:

```bash
export OPENAI_API_KEY=your-api-key-here
```

Or you can automate it by adding the above line to your .bashrc (Linux) or .bash_profile (Mac).

### 3. Build the Project

Navigate to the root directory of the project and execute:

```bash
cargo build --release
```

This will generate an optimized executable in ./target/release/.

### Usage
To run the program:

```bash
./target/release/mansplain [COMMAND]
```

