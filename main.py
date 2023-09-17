import inspect
import rust_python_example
from rust_python_example.types import MyStr
#from rust_python_example import MyProgram

expressions = [
    'b && (c == "string")',
    'b && c == "string"',
    'c == "string" && b',
]

a: MyStr = "my string"

for expression in expressions:
    # Compile expression
    program = rust_python_example.MyProgram(expression)

    # Evaluate expression
    result = program.evaluate({"a": 1, "b": True, "c": "string"})
    print(inspect.signature(program.evaluate))

    # Print results
    print(f"{result} <= {expression}")

a = 1
b = True
c = "string"

if (a == 1) and b and (c == "string"):
    print("This is true")


if a == 1 and b and c == "string":
    print("This is also true")
