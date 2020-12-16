"""This is the model interface that links to the web based GUI."""
from enum import Enum
from shapely.geometry import shape
from dataclasses import dataclass, replace
from random import randint
from typing import Callable, List, Tuple

from geojson.feature import FeatureCollection
import cellular_automata

# from pyUrbanCap.Config.Config_Shape import Config
# from pyUrbanCap.Model_State.Model_State import CellState, generate_random_cells
# from pyUrbanCap.run_model import run_iteration as run_model_iteration

# from pyUrbanCap.cell_processes.business import cell_calculate_business_change

# from pyUrbanCap.cell_processes.population import (
#     cell_calculate_pop_attraction,
#     cell_calculate_population_egress,
#     cell_calculate_population_birth_rate,
#     cell_calculate_population_death_rate,
#     cell_calculate_population_change,
# )

# from pyUrbanCap.cell_processes.construction import (
#     cell_calculate_construction_demand_change,
#     cell_calculate_construction,
#     cell_calculate_construction_capacity_change
# )

# PLACEHOLDER


@dataclass
class CellState:
    population: int = 0

# PLACEHOLDER


@dataclass
class Config:
    foo: int = 0


MODEL_LAYERS = [
    {
        "uid": "population",
        "label": "Population"
    },
    # {
    #     "uid": "residential_capacity",
    #     "label": "Residential Capacity"
    # },
    # {
    #     "uid": "residential_demand",
    #     "label": "Residential Demand"
    # },
    # {
    #     "uid": "population_attraction",
    #     "label": "Population attraction"
    # },
    # {
    #     "uid": "total_jobs",
    #     "label": "Jobs"
    # },
    # {
    #     "uid": "office_capacity",
    #     "label": "Offices"
    # },
    # {
    #     "uid": "office_demand",
    #     "label": "Office Demand"
    # }
]


GLOBAL_PARAMS = [i['uid'] for i in MODEL_LAYERS]


def invalid_action_fn(cell_data: CellState) -> CellState:
    print('Invalid Action')
    return cell_data


invalid_action = {'uid': 'invalid', 'label': 'Invalid',
                  'type': 'invalid', 'fn': invalid_action_fn}


class ActionType(Enum):
    BUTTON = 'button'


@dataclass
class Action:
    uid: str
    label: str
    type: ActionType
    fn: Callable[[CellState, dict], CellState]
    group: str = 'NONE'


CELL_ACTIONS = {a.uid: a for a in [
    # Population
    Action('setpop99999', 'Set pop to 99999', ActionType.BUTTON,
           lambda cellData: replace(cellData, population=99999), group='Population'),
    Action('setpop150', 'Set pop to 150', ActionType.BUTTON,
           lambda cellData: replace(cellData, population=150), group='Population'),
    Action('setpop99', 'Set pop to 99', ActionType.BUTTON,
           lambda cellData: replace(cellData, population=99), group='Population'),
    Action('setpop50', 'Set pop to 50', ActionType.BUTTON,
           lambda cellData: replace(cellData, population=50), group='Population'),
]}


def get_global_sum(data: List[object], attr) -> float:
    """Get the sum of an attribute from a list of objects."""
    return sum([getattr(c, attr) for c in data])


def get_attr_sum_map(
        prev: dict,
        data: List[object],
        fields: List[str],
        history_count: int = 10,
) -> dict:
    """Combine the sums of each field in data."""
    return {
        **prev,
        **{
            f: (prev[f] + [get_global_sum(data, f)])[-history_count:] for f in fields
        }
    }


def get_model_layers():
    return MODEL_LAYERS


class ControlTypes(Enum):
    """Control types for frontend client controls."""

    SLIDER = 'slider'


@dataclass
class UIControl:
    """Ui control fields data."""

    uid: str
    label: str
    control_type: ControlTypes
    type: str
    min: float = None
    max: float = None

    def parse_for_client(self):
        """Parse the class for frontend client."""
        return {
            'uid': self.uid,
            'label': self.label,
            'controlType': self.control_type.value,
            'type': self.type,
            'min': self.min,
            'max': self.max,
        }


def get_config_fields():
    return [
        # UIControl('growth_rate', 'Growth Rate', ControlTypes.SLIDER, 'number', min=-200, max=200),
    ]


def setup_state(initial_geo_data: FeatureCollection, seed: int = 0):
    feature_centroids = [
        shape(f.geometry).centroid for f in initial_geo_data.features
    ]
    ids = list(range(len(feature_centroids)))
    positions = [list(f.centroid.coords)[0] for f in feature_centroids]
    # TODO: Shouldn't hard code feature property identifier here
    labels = [f.properties.get('NAME_B', f'cell_{i}')
              for i, f in enumerate(initial_geo_data.features)]

    # TODO: Get initial cell data from rust
    # initial_cell_data = generate_random_cells(ids, labels, positions, seed)
    initial_cell_data = []

    initial_global_data = {
        'totals': {k: [get_global_sum(initial_cell_data, k)] for k in GLOBAL_PARAMS},
    }
    return initial_cell_data, initial_global_data


def get_config(ui_config, model_config=None, **kwargs) -> Config:
    return replace(model_config, **kwargs) \
        if model_config is not None \
        else Config(**kwargs)


def get_cell_actions():
    return [{
        "uid": a.uid,
        "label": a.label,
        "type": a.type.value,
        "group": a.group,
    } for a in CELL_ACTIONS.values()]


def action_runner(cell_data: CellState, action_id: str, **kwargs) -> CellState:
    return CELL_ACTIONS.get(action_id, invalid_action).fn(cell_data, **kwargs)


def run_iteration(
    config: Config,
    cell_data: List[CellState],
    global_data: dict,
) -> Tuple[List[CellState], dict]:

    cells_out = cellular_automata.run_iteration(
        # cell_data,
    )
    return cells_out

    # TODO: Get all data from run_iteration

    # cells_out, global_data_out, network_map_out = cellular_automata.run_iteration(
    #     # config,
    #     # processes,
    #     cell_data,
    #     global_data,
    #     network_map,  # TODO: Implement network map between iterations
    # )
    # global_data_out['totals'] = {
    #     k: [*global_data['totals'][k][-20:], get_global_sum(cells_out, k)]
    #     for k in GLOBAL_PARAMS if k in global_data['totals']}

    # global_data_out['network_map'] = network_map_out
    # return cells_out, global_data_out


if __name__ == "__main__":
    out = run_iteration({}, {}, {})
    print(out)
