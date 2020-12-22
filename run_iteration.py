from cellular_automata import run_iteration, CellStatePy

initial_cell_data = [
    CellStatePy(0, (0, 1), 20),
]

result = run_iteration(initial_cell_data)

print(result)
population_a = result[0].population
print(result[0].population)
print(type(population_a))
