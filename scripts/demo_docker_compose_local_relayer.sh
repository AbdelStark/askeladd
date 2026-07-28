#!/bin/bash
# One-command Askeladd demo: local Nostr relay + prover agent + customer,
# all wired together with docker-compose.

set -e

# Pick docker compose v2 if available, else the v1 binary.
if docker compose version &>/dev/null; then
    COMPOSE="docker compose"
elif command -v docker-compose &>/dev/null; then
    COMPOSE="docker-compose"
else
    echo "docker compose is required but not installed."
    exit 1
fi

# Function to stop all containers and exit
cleanup() {
    echo "Stopping all containers..."
    $COMPOSE down
    exit 0
}

# Trap Ctrl+C and call cleanup
trap cleanup INT

# Ensure we're in the repository root (where docker-compose.yml lives)
cd "$(dirname "$0")/.."

# Build (if needed) and start the containers
$COMPOSE up -d --build

# Wait for the relay to accept connections
echo "Waiting for the Nostr relay to be ready..."
if command -v curl &>/dev/null; then
    until curl -s -o /dev/null http://localhost:8080; do sleep 1; done
else
    # No curl on the host: give the relay a moment to boot.
    sleep 5
fi

# Function to display logs with a specific color
show_logs() {
    local container_name=$1
    local color=$2
    $COMPOSE logs -f "$container_name" | sed "s/^/$(tput setaf $color)[$container_name] /"
}

# Run log displays in the background
show_logs nostr-relay 1 &
show_logs dvm-service-provider 2 &
show_logs dvm-customer 3 &

# Wait for user input to stop
echo "Demo is running. Press Enter to stop..."
read -r

# Cleanup and exit
cleanup
