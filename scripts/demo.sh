#!/bin/bash

# Function to check if tmux is installed
check_tmux() {
    if ! command -v tmux &> /dev/null; then
        echo "tmux is not installed. Please install tmux and try again."
        exit 1
    fi
}

# Function to run the demo
run_demo() {
    # Create a new tmux session
    tmux new-session -d -s askeladd

    # Split the window vertically
    tmux split-window -h

    # Run the DVM Service Provider in the left pane
    tmux send-keys -t 0 './target/release/dvm_service_provider' C-m

    # Wait for 5 seconds to allow the Service Provider to start up
    sleep 3

    # Run the DVM Customer in the right pane
    tmux send-keys -t 1 './target/release/dvm_customer' C-m

    # Attach to the tmux session
    tmux attach-session -t askeladd
}

# Main execution
check_tmux
run_demo