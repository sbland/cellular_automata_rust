from cellular_automata import run_iteration, CellStatePy

initial_cell_data = [
    CellStatePy(i, (0, 1), 20)
    for i in range(100)]

result = run_iteration(initial_cell_data)

print(result)
population_a = result[0].population
print(result[0].population)
print(type(population_a))

print('Run Multiple Iterations')
cell_data = initial_cell_data
for i in range(100):
    cell_data = run_iteration(cell_data)
    population_a = cell_data[0].population
print(population_a)
print('Complete')
