import inspect
import pycel
from pycel.types import CelBool, CelInt, CelFloat, CelString, CelList, CelMap

expressions = [
    'b && (c == "string") && f >= 3.14 && a in g && o.id == d',
    'b && c == "string" && f >= 3.14',
    'c == "string" && b && f >= 3.14',
]

for expression in expressions:
    # Compile expression
    program = pycel.CelProgram(expression)

    # Evaluate expression
    result = program.evaluate({
        "a": CelInt(1),
        "d": CelInt(2),
        "b": CelBool(True),
        "c": CelString("string"),
        "f": CelFloat(3.15),
        "g": CelList([CelInt(1)]),
        "o": CelMap({"id": CelInt(2)}),
    })
    print(inspect.signature(program.evaluate))

    # Print results
    print(f"{result} <= {expression}")
