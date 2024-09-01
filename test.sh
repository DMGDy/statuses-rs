#!/usr/bin/env bash

# Read MemTotal and MemAvailable from /proc/meminfo
mem_total=$(grep '^MemTotal:' /proc/meminfo | awk '{print $2}')
mem_available=$(grep '^MemAvailable:' /proc/meminfo | awk '{print $2}')

# Calculate used memory
mem_used=$((mem_total - mem_available))

# Calculate percentage of used memory
percentage=$(awk "BEGIN {printf \"%.2f\", ($mem_used / $mem_total) * 100}")

# Convert to GB for readability
total_gb=$(awk "BEGIN {printf \"%.2f\", $mem_total / 1024 / 1024}")
used_gb=$(awk "BEGIN {printf \"%.2f\", $mem_used / 1024 / 1024}")

# Print the results
echo "Memory Usage:"
echo "Total: ${total_gb} GB"
echo "Used: ${used_gb} GB"
echo "Used Percentage: ${percentage}%"
