#!/usr/bin/env bash

set -e

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cargo run --example plot_neuron | python3 "$SCRIPTPATH"/plot.py
