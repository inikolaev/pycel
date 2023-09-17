# rust-python-example

An example of a Python wrapper for Common Expression Language [interpreter](https://github.com/clarkmcc/cel-rust) written in Rust.

## Development

You would need to have Rust toolchain installed and Python virtual environment created and activated.
Otherwise `maturin` will not be able to install the package,

```bash
# Create new virtual environment
pyenv virtualenv 3.11 rust-python-example

# And activate it
pyenv activate

# Compile extension
maturin develop

# Run a test
python main.py
```
