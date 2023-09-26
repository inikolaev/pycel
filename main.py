import inspect
import rust_python_example
from rust_python_example.types import CelBool, CelInt, CelFloat, CelString

expressions = [
    'b && (c == "string") && f >= 3.14',
    'b && c == "string" && f >= 3.14',
    'c == "string" && b && f >= 3.14',
]

for expression in expressions:
    # Compile expression
    program = rust_python_example.MyProgram(expression)

    # Evaluate expression
    result = program.evaluate({"a": CelInt(1), "b": CelBool(True), "c": CelString("string"), "f": CelFloat(3.15)})
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


def get_value() -> str:
    import random
    return str(random.randint(1, 9)) + "".join(str(random.randint(0, 9)) for _ in range(0,5))

expression = "a in [" + ",".join(get_value() for _ in range(1000)) + "]"
print(expressions)
program = rust_python_example.MyProgram(expression)

import time
start = time.time()
for _ in range(1000):
    program.evaluate({"a": CelInt(int(get_value()))})
end = time.time()
print(end - start)