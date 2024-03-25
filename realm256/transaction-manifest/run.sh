#!/bin/bash

# The script should receive the transaction ID used to deploy dApp
# Check if the number of arguments is not exactly 1
if [ "$#" -ne 1 ]; then
    echo "Usage: run <instantiate_transaction_id>"
    exit 1
fi

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR
echo $SCRIPT_DIR
python3 -m pip install -r $SCRIPT_DIR/requirements.txt > /dev/null 2>&1
python3 config_and_manifest_generator.py "$1"