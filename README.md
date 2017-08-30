# Description
A chatbot that could be used to schedule ad-hoc rowing practices.

# Development status
Currently suspended due to Diesel.rs having trouble associating tables.

# How to build:

* `docker build -t archrustdevenv .`
* `docker network create --driver bridge isolated_nw`
* `docker run --name rustDev -v ~/code/rustSms:/app --network=isolated_nw -p 8080:8080 -itd archrustdevenv /bin/bash`
* `docker run --name some-postgres -e POSTGRES_PASSWORD=password -d --network=isolated_nw postgres`
* run `docker network inspect isolated_nw` and grab the IPv4Address and use it to fill out the address of the postgres address in the .env file
