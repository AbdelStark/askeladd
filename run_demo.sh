#!/bin/bash

# Function to stop all containers and exit
cleanup() {
    echo "Stopping all containers..."
    docker-compose down
    exit 0
}

# Trap Ctrl+C and call cleanup
trap cleanup INT

# Ensure we're in the correct directory
cd "$(dirname "$0")"

# Build and start the containers
#docker-compose up --build -d
docker-compose up -d

# Wait for services to be ready
echo "Waiting for services to be ready..."
docker-compose run --rm user-cli /bin/sh -c "until wget -q --spider http://nostr-relay:8080; do sleep 1; done"

# Function to display logs with a specific color
show_logs() {
    local container_name=$1
    local color=$2
    docker-compose logs -f "$container_name" | sed "s/^/$(tput setaf $color)[$container_name] /"
}

# Run log displays in the background
show_logs nostr-relay 1 &
show_logs prover-agent 2 &
show_logs user-cli 3 &

# Wait for user input to stop
echo "Demo is running. Press Enter to stop..."
read

# Cleanup and exit
cleanup