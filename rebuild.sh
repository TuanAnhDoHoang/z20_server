#!/bin/sh
docker stop z20_rserver && docker rm z20_rserver
docker build -t z20_rserver:latest .
docker run -d --name z20_rserver   -p 5000:5000   --env-file .env   z20_rserver:latest   
curl http://localhost:5000/api/v1