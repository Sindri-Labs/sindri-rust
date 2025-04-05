# Sindri Rust CLI

A command-line interface for interacting with Sindri's API distributed as a cargo binary.
An alternative Sindri CLI, written in typescript and distributed as a npm package, can be found [here](https://github.com/Sindri-Labs/sindri-js).

## Installation

Install the latest Sindri rust CLI via:

```bash
cargo install sindri-cli --force --locked
```

## Usage

### Login

Most functionalities within the Sindri Rust SDK and CLI require you to supply your API key.
This method allows you to create an API key by providing your Sindri username and password via:

```bash
cargo sindri login [OPTIONS]
```

#### Options
- `--username <USERNAME>`: Sindri username (optional, will prompt if not provided)
- `--password <PASSWORD>`: Sindri password (optional, will prompt if not provided)
- `--keyname <KEYNAME>`: Name to identify your new key (optional, will prompt if not provided)
- `--teamname <TEAMNAME>`: Sindri team which the key should be created for (optional, will prompt if not provided)
- `--base-url <URL>`: Sindri API base URL (overrides SINDRI_BASE_URL env var)

The login command will prompt for your Sindri credentials (if not provided via options) and allow you to select a team to generate an API key for.

After successful login, you can use the generated API key by either:
- Setting the `SINDRI_API_KEY` environment variable
- Using the `--api-key` flag with any `cargo sindri` command


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
