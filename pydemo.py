from dataclasses import dataclass
from typing import List
import cellular_automata
from timeit import repeat

from run_model import Actions, run_iteration as run_model_iteration, Config, CellState


def cell_example_process(
    config: Config,
    cell_state: CellState,
    neighbour_cells: List[CellState],
):
    return [(cell_state.uid, {'population': (Actions.ADD, 5)})]


def run_iteration(
    config: Config,
    cell_data: List[CellState],
    global_data: dict,
):
    processes = [
        cell_example_process,
    ]
    network_map = global_data.get('network_map', None)
    cells_out, global_data_out, network_map_out = run_model_iteration(
        config,
        processes,
        cell_data,
        global_data,
        network_map,  # TODO: Implement network map between iterations
    )
    # global_data_out['totals'] = {
    #     k: [*global_data['totals'][k][-20:], get_global_sum(cells_out, k)]
    #     for k in GLOBAL_PARAMS if k in global_data['totals']}

    global_data_out['network_map'] = network_map_out
    return cells_out, global_data_out


cell_data = [CellState(i, 10, (0, i), (0, 0)) for i in range(100)]


out = min(repeat(lambda: run_iteration(
    Config(),
    cell_data,
    {'network': []},
), number=10, repeat=3))
print(f'python {out} seconds')


out = min(repeat(lambda: cellular_automata.demo_run(), number=10, repeat=3))
print(f'rust {out} seconds')

# a = cellular_automata.sum_as_string(3, 3)
# print(a)

# b = cellular_automata.demo_run()
# print(b)
