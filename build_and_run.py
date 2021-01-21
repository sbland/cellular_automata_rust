import subprocess
subprocess.check_call(['sh', './scripts/_test_rust.sh'])
subprocess.check_call(['sh', './scripts/_build_rust.sh'])


def run():
    from cellular_automata_py.run_iteration import demo_run
    demo_run()


run()
