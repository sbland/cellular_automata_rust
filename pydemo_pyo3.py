from cellular_automata import (
    sum_as_string,
    sum_as_string_data,
    sum_as_string_data_b,
    sum_as_string_data_c,
    Data,
)
print(sum_as_string(3, 3))


class DataB:
    def __init__(self) -> None:
        self.a = 3
        self.b = 9


print(sum_as_string_data(Data(3, 4)))
print(sum_as_string_data_b({'a': 1, 'b': 2}))
print(sum_as_string_data_c(DataB()))
