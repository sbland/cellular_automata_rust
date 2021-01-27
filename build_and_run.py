"""Test Build test again then run a demo pass."""

import subprocess


def run():
    subprocess.check_call(['sh', './scripts/_test_rust.sh'])
    subprocess.check_call(['sh', './scripts/_build_rust.sh'])
    # // TODO: Implement pytest
    # pytest.main()

    from cellular_automata_py.run_iteration import demo_run
    demo_run()


if __name__ == "__main__":
    run()
