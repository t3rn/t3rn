#!/bin/bash

# Include the script with the decoding functions
source decode_scale.sh

# Initialize variables for the new options
speed_mode="Fast"  # Default value, if needed
as_utility_batch=false
as_sequential_tx=false
as_multi_sfx=false
repeat=1  # Default repetition
repeat_interval=0  # Default interval, 0 means no interval
remote_reward=false

# Function to set the order path
set_order_path() {
    clear
    echo "+---------------------------------------------------------+"
    echo "|                   Select Order Path                     |"
    echo "+---------------------------------------------------------+"
    echo "| 1. Local - Local                                        |"
    echo "| 2. Remote with Local Reward                             |"
    echo "| 3. Remote with Remote Reward                            |"
    echo "| 4. Bridging                                             |"
    echo "+---------------------------------------------------------+"
    echo "Choose a path: "
    read -r path_choice

    case "$path_choice" in
        1)
            # Prompt for source and destination if needed
            echo "Confirm Destination==Source: (current: $source)"
            read -r source_input
            source=${source_input:-${source}}
            dest=$source  # Assuming $dest is already set somewhere in the script
            ;;
        2)
            # Prompt for source and destination if needed
            echo "Enter Destination:"
            read -r dest
            ;;
        3)
            # Prompt for source and destination if needed
            echo "Enter Destination:"
            read -r dest
            reward_asset=true  # Placeholder, set to the appropriate remote identifier
            ;;
        4)
            # Bridging option, set reward_asset equal to target_asset
            echo "Enter Destination:"
            read -r dest
            echo "Enter Target Asset ID for Bridging:"
            read -r target_asset
            reward_asset=$target_asset
            echo "Enter Amount to Bridge:"
            read -r amount
            echo "Enter Max Reward you wanna offer as a bridging reward:"
            read -r max_reward
            ;;
        *)
            echo "Invalid option. Please try again."
            return
            ;;
    esac
}

# Function to set scheduling and repetition parameters
set_schedule_and_repetitions() {
    echo "Send as a utility::batch call? (y/n):"
    read -r input
    [[ "$input" == "y" ]] && as_utility_batch=true

    echo "Send as a sequence of transactions? (y/n):"
    read -r input
    [[ "$input" == "y" ]] && as_sequential_tx=true

    echo "Send as an XTX containing multiple of SFXs? (y/n):"
    read -r input
    [[ "$input" == "y" ]] && as_multi_sfx=true

    echo "Repeat the transaction? Enter number of repetitions:"
    read -r repeat
    repeat=${repeat:-1}  # Default to 1 if no input

    echo "Repeat the transaction every x seconds? Enter interval:"
    read -r repeat_interval
    repeat_interval=${repeat_interval:-0}  # Default to 0 if no input
}

# Function to prompt user for command line options
set_command_line_options() {
    echo "Enter Signer (default: //Alice):"
    read -r signer
    signer=${signer:-"//Alice"}

    echo "Enter Target Account (default: //Bob):"
    read -r target_account
    target_account=${target_account:-"//Bob"}

    echo "Enter Speed Mode (Instant, Fast, Normal, Slow) (default: Instant):"
    read -r speed_mode
    speed_mode=${speed_mode:-"Instant"}  # Default to "Fast" if no input

    echo "Enter Order Asset (current: ${target_asset}):"
    read -r target_asset_input
    target_asset=${target_asset:-${target_asset_input}}

    echo "Enter Order Amount (current: ${target_amount}):"
    read -r target_amount_input
    target_asset=${target_asset:-${target_amount_input}}

    echo "Enter Reward Asset (will be subtracted from your account) (current: ${reward_asset}):"
    read -r reward_asset_input
    reward_asset=${reward_asset:-${reward_asset_input}}

    echo "Enter Reward Amount (will be subtracted from your account) (current: ${max_reward}):"
    read -r max_reward_input
    max_reward=${max_reward:-${max_reward_input}}

    # set insurance to 5% of max reward
    insurance=$(echo "scale=2; $max_reward * 0.05" | bc)
    insurance=$(printf '%.2f\n' $insurance)
    echo "Enter Insurance (will be subtracted from your executor's account who picks up your order) (default is set to 5% of max reward: ${insurance}):"
    read -r insurance_input
    insurance=${insurance:-${insurance_input}}

    # ... continue with other parameters similarly
    # Assuming all parameters have been read and set in variables...

    cli_command="pnpm cli writer --signer $signer --target-account $target_account --target-asset $target_asset --target-amount $target_amount --reward-asset $reward_asset --max-reward $max_reward --insurance $insurance --speed-mode $speed_mode --endpoint $endpoint --dest $dest --source $source"
    echo "Command to execute:"
    echo $cli_command
}

# Function to load registered assets
load_registered_assets() {
    result_json=$(curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": ["0xe1cd48a2f22b658ec09db2a8c88f150d3872789d0b96a774bacb2224eba33f22"]}' https://rpc.t0rn.io | jq -r ".result")
    # Decode the SCALE-encoded array of u32 values
    registered_assets=$(decode_scale_u32 "$result_json")
}

# Function to load registered targets
load_registered_targets() {
    result_json=$(curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": ["0xe1cd48a2f22b658ec09db2a8c88f150db18b263656ab6453555c4cfe566a8f2d"]}' https://rpc.t0rn.io | jq -r ".result")
    # Decode the SCALE-encoded array of u32 values
    registered_targets=$(decode_scale_ascii "$result_json")
}

# Load assets and targets at the start of the script
load_registered_assets
load_registered_targets

#
# Main menu loop
while true; do
    clear
    echo "+---------------------------------------------------------+"
    echo "|                    Fast Writer CLI                      |"
    echo "+---------------------------------------------------------+"
    echo "| 1. Set SFX Order Details                                |"
    echo "| 2. Select Order Source                                  |"
    echo "| 3. Select Order Path                                    |"
    echo "| 4. Display Registered Targets                           |"
    echo "| 5. Display Registered Asset IDs                         |"
    echo "| 6. Submit Order                                         |"
    echo "| 7. Select schedule & repetitions                        |"
    echo "| 8. Exit                                                 |"
    echo "+---------------------------------------------------------+"8
    echo "Choose an option: "

    read -r option

    case "$option" in
        1)
            set_command_line_options
            ;;
        2)
            clear
            echo "+---------------------------------------------------------+"
            echo "|                   Select Order Source                   |"
            echo "+---------------------------------------------------------+"
            echo "| A. sepl - Ethereum Smart Contract @ Sepolia             |"
            echo "| B. eth2 - Ethereum Smart Contract @ Sepolia             |"
            echo "| C. 0x03030303 - t3rn Parachain @ Polkadot               |"
            echo "| D. 0x03030303 - t0rn Parachain @ Testnet Rococo         |"
            echo "| E. 0x03030303 - t1rn Parachain @ Kusama                 |"
            echo "| F. 0x03030303 - t0rn's Local Parachain @ Locally        |"
            echo "+---------------------------------------------------------+"
            echo "Select A, B, C, D, or E: "
            read -r path_choice

            case "$path_choice" in
                1)
                    source="sepl"
                    endpoint="https://eth-sepolia.public.blastapi.io"
                   ;;
                2)
                    source="eth2"
                    endpoint="https://mainnet.infura.io/v3/"
                   ;;
                3)
                    source="0x03030303"
                    endpoint="wss://rpc.t3rn.io"
                    ;;
                4)
                    source="0x03030303"
                    endpoint="wss://rpc.t0rn.io"
                    ;;
                5)
                    source="0x03030303"
                    endpoint="wss://rpc.t1rn.io"
                    ;;
                6)
                    source="0x03030303"
                    endpoint="ws://127.0.0.1:9944"
                    ;;
                *)
                    echo "Invalid option. Please try again."
                    return
                    ;;
            esac
            ;;

        3)
            clear
            echo "+---------------------------------------------------------+"
            echo "|                   Select Order Path                     |"
            echo "+---------------------------------------------------------+"
            echo "| 1. Local - Local                                        |"
            echo "| 2. Remote with Local Reward                             |"
            echo "| 3. Remote with Remote Reward                            |"
            echo "| 4. Bridging                                             |"
            echo "+---------------------------------------------------------+"
            echo "Choose a path: "
              set_order_path
            echo "Updated command to execute:"
            cli_command="pnpm cli writer --signer $signer --target-account $target_account --target-asset $target_asset --target-amount $target_amount --reward-asset $reward_asset --max-reward $max_reward --insurance $insurance --speed-mode $speed_mode --endpoint $endpoint --dest $dest --source $source"
            echo $cli_command
            ;;
        4)
            echo "Registered Targets:"
            echo "$registered_targets"
            ;;
        5)
            echo "Registered Asset IDs:"
            echo "$registered_assets"
            ;;
        6)
            # Here you would call the TypeScript CLI with the constructed command
            # Construct the command with the new scheduling options
            cli_command="pnpm cli writer --signer $signer --target-account $target_account --target-asset $target_asset --target-amount $target_amount --reward-asset $reward_asset --max-reward $max_reward --insurance $insurance --speed-mode $speed_mode --endpoint $endpoint --dest $dest"

            # Append scheduling options to the command
            [[ "$as_utility_batch" == true ]] && cli_command+=" --as-utility-batch"
            [[ "$remote_reward" == true ]] && cli_command+=" --remote-reward"
            [[ "$as_sequential_tx" == true ]] && cli_command+=" --as-sequential-tx"
            [[ "$as_multi_sfx" == true ]] && cli_command+=" --as-multi-sfx"
            cli_command+=" --repeat $repeat"
            [[ "$repeat_interval" -gt 0 ]] && cli_command+=" --repeat-interval $repeat_interval"

            eval $cli_command
            ;;
        7)
            set_schedule_and_repetitions
            ;;
        8)
            exit 0
            ;;
        *)
            echo "Invalid option. Please try again."
            ;;
    esac

    echo "Press any key to continue..."
    read -r -n 1
done