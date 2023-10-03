#!/bin/bash

# Fetch all services from docker-compose.yml
services=$(docker-compose config --services)
number_of_services=$(docker-compose config --services | wc -l)

while true; do 
    number_of_containers=$(docker ps -q | wc -l)
    # wait until all containers are up and running
    echo $number_of_containers $number_of_services
    if [ $number_of_containers -eq $number_of_services ]; then
        break
    fi
done

# Loop over each service and create the start file
for service in $services; do
    docker exec $service touch /tmp/start.txt &
done

# Wait for all background jobs to complete
wait
