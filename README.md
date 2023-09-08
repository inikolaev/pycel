# rust-python-example

An example of a Python wrapper for a library written in Rust.

## Development

You would need to have Rust toolchain installed and Python virtual environment created and activated.
Otherwise `maturin` will not be able to install the pckage,

```
maturin develop
python -c "import rust_python_example; program = rust_python_example.MyProgram('1 == 1'); print(program.evaluate());"
```
