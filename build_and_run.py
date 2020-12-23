import subprocess
from cellular_automata_py.run_iteration import demo_run
subprocess.call(['sh', './scripts/_build_rust.sh'])
demo_run()
