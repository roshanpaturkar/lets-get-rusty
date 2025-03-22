#!/bin/bash

# Delete all the target directories and files including cargo.lock from all the current and subdirectories
find . -name "target" -type d -exec rm -rf {} +
find . -name "Cargo.lock" -type f -exec rm -f {} +
