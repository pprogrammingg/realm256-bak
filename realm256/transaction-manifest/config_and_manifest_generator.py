# This script takes an dApp instantiation transaction id, queries the transaction
# to get the relevant dApp instant info, updated <app_root>/config/global_config.json
# with relevant values and generates new transaction manifests for testing.
# global_config.json is also used for automatic jobs so all updates to this file 
# affect both transaciton_manifests and automatic jobs.

import json
import os
import requests
import sys

# TODO use ../../config/config_loader.pyto refactor this instead of 
# repeating loading code in each automatic job script
CONFIG_FILE_PATH = '../config/global_config.json'
def load_config():
    if os.path.exists(CONFIG_FILE_PATH):
        try:
            with open(CONFIG_FILE_PATH, 'r') as file:
                data = json.load(file)
                return data
        except (json.JSONDecodeError, KeyError, FileNotFoundError) as e:

            message = f"Failed to obtain {CONFIG_FILE_PATH}"
            print(f'''
                Script: [manifest_generator.py],
                Function: [load_config()]
                Message: {message}''')

            raise(Exception(f"{message}"))   
    else:
        raise(Exception(f"{CONFIG_FILE_PATH} does not exist!"))

# get component and validators info from here
GLOBAL_CONFIG = load_config()

# change to appropriate network if needed
GATEWAY_URL_BASE = GLOBAL_CONFIG['gateway_url_base']['stokenet']
HEADERS = {"Content-Type": "application/json; charset=utf-8"}

def get_transaction_affected_global_entities(instantiate_transaction_id: str):

    GET_TX_DETAILS_API_PATH =  "/transaction/committed-details"
    url = GATEWAY_URL_BASE + GET_TX_DETAILS_API_PATH
    payload = {
        "intent_hash": f"{instantiate_transaction_id}",
        "opt_ins": {"affected_global_entities" : True}
    }
    
    tx_details_response = requests.post(url, json=payload, headers=HEADERS)

    if tx_details_response.status_code == 200:
        data = tx_details_response.json()
        affected_global_entities = data['transaction']['affected_global_entities']
        entities_len = len(affected_global_entities)
        # Take only component and new resource info
        dapp_instant_values_array = affected_global_entities[1:entities_len-1]
        # put instantiation tx id at the beginning
        dapp_instant_values_array.insert(0, instantiate_transaction_id)

        #print(json.dumps(dapp_instant_values_array, indent=4))
        return dapp_instant_values_array
    else:
        message = f"Error obtaining affected_global_entitie for transaction id: {instantiate_transaction_id}."
        print(f'''
            API Error: [{url}],
            Script: [manifest_generator],
            Function: [get_transaction_affected_global_entities()],
            Status: [{tx_details_response.status_code}],
            Message: {message}''')
        
        raise Exception(f"{message}")

def update_global_config(dapp_instant_values_array):
    dapp_instant_info_from_config = GLOBAL_CONFIG["dapp_instant_info"]

    # Get the keys
    keys = dapp_instant_info_from_config.keys()

    # Update values using zip to iterate through keys and dapp_instant_info
    for key, new_value in zip(keys, dapp_instant_values_array):
        GLOBAL_CONFIG['dapp_instant_info'][key] = new_value

    #print(json.dumps(GLOBAL_CONFIG, indent=4))
    return GLOBAL_CONFIG

def update_transaction_manifest_files(values_for_manifest_generation):
    # print(f"Updating global config with: {json.dumps(values_for_manifest_generation, indent=4)}")
    # Directory paths
    TEMPLATES_FOLDER = './templates'
    OUTPUT_FOLDER = './manifests'

    # Remove files inside the output folder if it exists
    if os.path.exists(OUTPUT_FOLDER):
        files = os.listdir(OUTPUT_FOLDER)
        for file in files:
            file_path = os.path.join(OUTPUT_FOLDER, file)
            os.remove(file_path)

    # Recreate the output folder
    os.makedirs(OUTPUT_FOLDER, exist_ok=True)

    # Iterate through each file in the templates folder
    for filename in os.listdir(TEMPLATES_FOLDER):
        if filename.endswith(".rtm"):
            input_file_path = os.path.join(TEMPLATES_FOLDER, filename)
            output_file_path = os.path.join(OUTPUT_FOLDER, filename)

            # Read the content of the file
            with open(input_file_path, 'r') as file:
                file_content = file.read()
            
            # Replace '<some_key_name>' with the value obtained from values_for_manifest_generation
            for key, value in values_for_manifest_generation.items():
                # print(f"Key: {key}, Value: {value}")
                file_content = file_content.replace(f"<{key}>", str(value))
            
            # Write the modified content to a new file in the output folder
            with open(output_file_path, 'w') as file:
                file.write(file_content)

def main(instantiate_transaction_id):
    print(f"Starting config_and_manifest_generator.py with transaction_id: {instantiate_transaction_id}")

    dapp_instant_values_array = get_transaction_affected_global_entities(instantiate_transaction_id)

    # given instantiate transaction outcomes, update global config with the correct value
    updated_global_config = update_global_config(dapp_instant_values_array)

    # overwrite global_config with updated values
    with open(CONFIG_FILE_PATH, 'w') as file:
        json.dump(updated_global_config, file, indent=4)

    # values_for_manifest_generation takes a subset GLOBAL_CONFIG in a flattened fashion so it is easier to traverse
    values_for_manifest_generation = {} 
    values_for_manifest_generation.update(GLOBAL_CONFIG['dapp_instant_info'])
    values_for_manifest_generation.update(GLOBAL_CONFIG['dapp_accounts'])
    values_for_manifest_generation.update({'xrd_resource_address': GLOBAL_CONFIG["xrd_resource_address"]})

    # print(f"values_for_manifest_generation {json.dumps(values_for_manifest_generation, indent=4)}")
    
    # update manifest files
    update_transaction_manifest_files(values_for_manifest_generation)

    print(f"Finished config_and_manifest_generator.py - updated global_config.json and generated manifests.")

if __name__ == "__main__":
    # Check if an argument was passed
    if len(sys.argv) > 1:
        argument_from_shell = sys.argv[1]  # First argument after the script name
        main(argument_from_shell)
    else:
        print("Please provide transaction id as an argument.")