"""Functions required to run the model."""
from copy import deepcopy

from dataclasses import dataclass, replace
from typing import Any, Dict, Generator, List, Callable, Tuple, Union
from shapely.geometry import Point
from enum import Enum, auto


@dataclass
class Config_Shape:
    foo: int = 0


@dataclass
class Config:
    foo: int = 0


@dataclass
class CellState:
    uid: int = 0
    population: int = 0
    pos_x: Tuple[float, float] = None
    pos_y: Tuple[float, float] = None


class Actions(Enum):
    ADD = 'add'
    SET = 'set'


def process_change(cell_data, k, v):
    action, value = v
    if action == Actions.ADD:
        return getattr(cell_data, k) + value
    if action == Actions.SET:
        return value
    raise ValueError(f'{action} is an invalid action')


def distance_between_points(
    pA: Tuple[float, float],
    pB: Tuple[float, float],
) -> float:
    return Point(pA).distance(Point(pB))


def get_neighbour_map(
    cells: List[CellState],
    network_data: Any,
) -> List[List[CellState]]:
    return [
        [c for c in cells if distance_between_points(
            (c.pos_x, c.pos_y), (cell.pos_x, cell.pos_y)) < 2]
        for cell in cells
    ]


def run_cell_processes(
    config: Config,
    processes: List[Callable[[Config, CellState, List[CellState]], List[tuple]]],
    cell: CellState,
    neighbours: List[CellState],
) -> Generator[Tuple, None, None]:
    """Run each cell process for the cell. Outputs a list of changes to the cell.

    These changes are applied later

    Parameters
    ----------
    config : Config
        Model config
    processes : List[Callable[[Config, CellState, List[CellState]], List[tuple]]]
        List of processes to run on the cell
    cell : CellState
        The target cell state
    neighbours : List[CellState]
        List of all neighbour cells

    Returns
    -------
    [type]
        [description]
    """
    cell_impacts = (r for f in processes for r in f(config, cell, neighbours))
    return cell_impacts


def apply_cell_changes(
    changes: Generator,
    cells: Dict[str, CellState],
):
    changes = list(changes)
    for c_id, change in changes:
        cell_data = cells[c_id]
        updated_data = {
            k: process_change(cell_data, k, v)
            for k, v in change.items()
        }

        # TODO: Can we do this without mutation?
        cells[c_id] = replace(
            cells[c_id],
            **updated_data
        )
    return cells


def flatten_changes(
    cell_updates: Generator[dict, None, None],
) -> Generator[dict, None, None]:
    return (i for d in cell_updates if d is not None for i in d if i is not None)


def run_iteration(
    config: Config,
    processes: List[Callable[[Config, CellState, List[CellState]], List[tuple]]],
    cells: List[CellState],
    global_data: dict,
    network_map: List[List[int]],
) -> Tuple[List[CellState], dict]:
    """Run a model iteration.

    Iteration steps:
        1. Run each process on each cell and get a list of changes
        2. Flatten the list of changes filtering out None values
        3. Apply the changes to the cells

    Parameters
    ----------
    - config : Config
        Model config
    - processes : List[Callable[[Config, CellState, List[CellState]], List[tuple]]]
        List of processes to run on each cell
    - cells : List[CellState]
        List of all cells
    - global_data: dict
        dictionary of global data
    - network_map: List[List[int]]
        a list of cell ids for each cell relating to its connected neighbour cells

    Returns
    -------
    Dict[str, CellState]
        outputs all cells after iteration
    """
    global_data_updated = deepcopy(global_data)  # TODO: Implement global data
    should_update_network_data = network_map is None

    updated_network_map = get_neighbour_map(cells, network_map) \
        if should_update_network_data else network_map
    cell_updates = (
        run_cell_processes(config, processes, cell, updated_network_map[i])
        for i, cell in enumerate(cells)
    )

    flattened_changes = flatten_changes(cell_updates)
    updated_cells = apply_cell_changes(flattened_changes, cells)
    return updated_cells, global_data_updated, updated_network_map


def run_cycles(
    config: Config,
    processes: List[Callable[[Config, CellState, List[CellState]], List[tuple]]],
    cellsdata: Dict[str, CellState],
    iterations_override: int = None,
):
    """Run model iterations.

    Parameters
    ----------
    config : Config
        Model config
    processes : List[Callable[[Config, CellState, List[CellState]], List[tuple]]]
        [description]
    cells : Dict[str, CellState]
        Dictionary mapping cell location id to cell data

    Returns (Tuple)
    -------
    Dict[str, CellState]
        Dictionary mapping cell location id to cell data
    List[List[CellState]]
        Log of all cell states at each step. Shape: (iterations, cells)
    """
    iterations = iterations_override if iterations_override is not None else config.iterations
    global_data = {}  # TODO: Implement global state
    logs = []
    cells = deepcopy(cellsdata)
    network_map = None
    for i in range(iterations):
        # print(f'iteration: {i}')
        cells, global_data, network_map = run_iteration(
            config,
            processes,
            cells,
            global_data,
            network_map,
        )
        logs.append(deepcopy(cells))
    return cells, logs


def get_model_iterator(
    cell_processes: List[Callable],
    config: Config_Shape,
    initial_cell_state: List[CellState],
) -> Generator:
    """Takes the model setup and outputs the model generator.

    """
    cell_data = deepcopy(initial_cell_state)
    while True:
        cell_data, global_data = run_iteration(
            config, cell_processes, cell_data)
        yield cell_data, global_data
