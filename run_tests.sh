#!/bin/bash
single_source code docs/tutorial.md tests/integration.rs rust test
single_source md docs/tutorial.md docs/tutorial_rendered.md 
cargo test
