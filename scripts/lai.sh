#!/usr/bin/env bash

show_help() {
  echo "Usage: lai -p <prompt>"
  echo "       command | lai -p <prompt>"
  echo ""
  echo "Options:"
  echo "  -p <prompt>    User prompt to include with the piped/inline data"
  echo "  -h             Show help"
}

# Defaults
prompt=""
input=""

# Parse flags
while getopts ":p:h" opt; do
  case $opt in
    p) prompt="$OPTARG" ;;
    h) show_help; exit 0 ;;
    \?) echo "Invalid option -$OPTARG" >&2; show_help; exit 1 ;;
  esac
done

# Read from stdin if piped
if [ ! -t 0 ]; then
  input=$(cat)
fi

# Check for required prompt
if [ -z "$prompt" ]; then
  echo "Error: Prompt (-p) is required." >&2
  show_help
  exit 1
fi

# Assemble JSON payload
json_payload=$(jq -n --arg prompt "$prompt" --arg input "$input" \
  '{message: ($prompt + "\n\n" + $input), mode: "chat"}')

# Send to AnythingLLM via cURL
response=$(curl -sS -w "\n%{http_code}" -X POST http://localhost:3001/api/v1/workspace/sysadmin/chat \
  -H 'accept: application/json' \
  -H 'Authorization: Bearer VG1Y609-GMWMC81-HFWR4HM-BRR9SZE' \
  -H 'Content-Type: application/json' \
  -d "$json_payload")

echo "$response" | sed '$d' | jq -r '.textResponse'
