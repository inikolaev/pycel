import inspect
import rust_python_example
from rust_python_example.types import CelBool, CelInt, CelFloat, CelString, CelList

def get_value() -> str:
    import random
    return str(random.randint(1, 9)) + "".join(str(random.randint(0, 9)) for _ in range(0,5))

expression = "a in [" + ",".join(get_value() for _ in range(10000)) + "]"
print(expressions)
program = rust_python_example.CelProgram(expression)

import time
start = time.time()
for _ in range(1000):
    program.evaluate({"a": CelInt(int(get_value()))})
end = time.time()
print(end - start)