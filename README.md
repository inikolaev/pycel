# rust-python-example

An example of a Python wrapper for Common Expression Language [interpreter](https://github.com/clarkmcc/cel-rust) written in Rust.

## Development

You would need to have Rust toolchain installed and Python virtual environment created and activated.
Otherwise `maturin` will not be able to install the package,

```bash
maturin develop
python -c "import rust_python_example; program = rust_python_example.MyProgram('1 == 1'); print(program.evaluate());"
```

Python example:
```python
import rust_python_example

# Compile expression
program = rust_python_example.MyProgram('1 == 1')

# Evaluate expression
print(program.evaluate())
```
