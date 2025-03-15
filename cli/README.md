# Sindri Rust CLI

A command-line interface for interacting with Sindri's API distributed as a cargo binary.
An alternative Sindri CLI, written in typescript and distributed as a npm package, can be found [here](https://github.com/Sindri-Labs/sindri-js).

## Installation

Install the latest Sindri rust CLI via:

```bash
cargo install sindri-cli --force --locked
```

## Usage

### Clone a Circuit

Retrieve the original source code that was uploaded to Sindri for a given project build via:

```bash
cargo sindri clone <CIRCUIT> < [OPTIONS]
```

#### Arguments
- `<CIRCUIT>`: UUID or project build identifier to clone

#### Options
- `--directory <DIR>`: Path where the circuit should be saved (defaults to circuit name)
- `--api-key <KEY>`: Sindri API key (overrides SINDRI_API_KEY env var)
- `--base-url <URL>`: Sindri API base URL (overrides SINDRI_BASE_URL env var)

### Deploy a Circuit

Upload your local DSL circuit or zkVM code to Sindri so that you can generate proofs via:

```bash
cargo sindri deploy <PATH> [OPTIONS]
```

#### Arguments
- `<PATH>`: Path to a local project directory or an archive file (.zip, .tar, .tar.gz, .tgz)

#### Options
- `--api-key <KEY>`: Sindri API key (overrides SINDRI_API_KEY env var)
- `--base-url <URL>`: Sindri API base URL (overrides SINDRI_BASE_URL env var)
- `--tags <TAGS>`: Optional comma-separated tags to identify the circuit
- `--meta <KEY1=VALUE1,KEY2=VALUE2>`: Optional metadata key-value pairs (comma-separated)

#### Example

```bash
# Deploy with tags and metadata
cargo sindri deploy ./my-circuit --tags test,v1 --meta version=1.0,env=staging --api-key=your-api-key

# Deploy using environment variables for authentication
export SINDRI_API_KEY="your-api-key"
cargo sindri deploy ./my-circuit
```

After successful deployment, the CLI will output the circuit's UUID and human-readable identifier which can be used for future proof requests.
