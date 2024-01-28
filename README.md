# pushover_cli
Small CLI to easily send notifications through Pushover.

## Usage
- `cargo build --release`
- Place a copy of `config_sample.toml` with the name `config.toml` next to the binary file
- Define both tokens in `config.toml`
- `pushover_cli send <Title> <Message>`

### Pushover API
https://pushover.net/api