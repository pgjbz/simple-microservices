# Simple microservices

This repo is create to improve my microservices skills, practice base skill with docker, kubernetes, rust, Java, etc.

Its based on this architecture

![Architecture](architecture.svg)

# IMPORTANT

Its a study repository, with focus on message, calls, docker and others communication.

Each microservice is made with a different language, except products and payment, they are written in Rust

Architecture and microservices are based on this playlist on Youtube: https://www.youtube.com/watch?v=n0rPo2LVmhI&list=PL5aY_NrL1rjuzBYy1Gro6IVDF1BPkPK_m

# Dependencies 

Only need a [docker](https://docs.docker.com/engine/install/ubuntu/) and [docker-compose](https://docs.docker.com/compose/install/)

# Build

To build microservices and docker images just run a make.sh with

```bash 
bash ./make.sh
```

or just if script have execution permission

```bash
./make.sh
```

and to up all microservices run

```bash
docker-compose up -d
```

to terminate all microservices just run:

```bash
docker-compose down
```

### Note

all images necessary are uploaded to my docker hub, so is not necessary to buid microservices and docker images