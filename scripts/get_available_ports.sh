#!/bin/bash

start_port=60011
end_port=65000
num_ports_needed=$1
available_ports=()
for port in $(seq $start_port $end_port); do
  lsof -i :$port &>/dev/null
  if [ $? -eq 1 ]; then
    available_ports+=($port)
    if [ ${#available_ports[@]} -eq $num_ports_needed ]; then
      break
    fi
  fi
done

if [ ${#available_ports[@]} -eq $num_ports_needed ]; then
  echo "${available_ports[@]}"
else
  exit 1
fi
