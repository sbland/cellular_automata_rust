import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="cellular_automata_pyinterface",
    version="0.2.1",
    author="sam bland",
    author_email="sam.bland@sei.org",
    description="Cellular Automata Agent model",
    setup_requires=["shapely", "geojson"],
    tests_require=["pytest"],
    extras_require={"test": ["pytest"]},
    packages=setuptools.find_packages(),
    package_dir={"cellular_automata_py": "cellular_automata_py"},
    classifiers=[
        "Programming Language :: Python :: 3.8",
        "Operating System :: OS Independent",
    ],
)
