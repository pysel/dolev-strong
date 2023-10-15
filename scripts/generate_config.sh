#!/bin/bash

# Array of available ports. Replace this with your dynamic array.
available_ports=($(bash ./scripts/get_available_ports.sh $1))

# Configuration file path
config_file="config.txt"

# Clear previous config file
> $config_file

# First port is for the leader
echo "127.0.0.1:${available_ports[0]} leader" >> $config_file

# Remaining ports are for followers
for i in "${available_ports[@]:1}"; do
  echo "127.0.0.1:$i follower" >> $config_file
done
