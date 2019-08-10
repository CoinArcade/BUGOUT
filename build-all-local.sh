#!/bin/bash

docker-compose build judge
docker-compose build changelog
docker-compose build history-provider
docker-compose build kafkacat
cd gateway && cargo clean && cd .. && docker-compose build gateway
docker-compose build kafkacat