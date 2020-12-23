from cellular_automata import run, CellStatePy, GlobalStatePy


def demo_run():
    print("""
----RUNNING DEMO----
    Running a demo version of the model
    """)
    initial_cell_data = [
        CellStatePy(i, (0, 1), 20)
        for i in range(100)]
    global_state = GlobalStatePy()

    cell_data, global_state, network_map = run.run_iteration(
        initial_cell_data, global_state)
    print('\n----First iteration Output----')

    population_a = cell_data[0].population
    print("cell_data[0].population:", cell_data[0].population)
    print("type(population_a):  ", type(population_a))
    print("global_state.iterations", global_state.iterations)

    print('\n----Runing Multiple Iterations-----')
    cell_data = initial_cell_data

    for i in range(100):
        cell_data, global_state, network_map = run.run_iteration(
            cell_data, global_state)
        population_a = cell_data[0].population
    print(population_a)
    print('Complete')
