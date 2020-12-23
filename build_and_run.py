import subprocess
from cellular_automata_py.run_iteration import demo_run
subprocess.check_call(['sh', './scripts/_build_rust.sh'])
demo_run()
