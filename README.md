# How to build:

* `docker build -t archrustdevenv .`
* `docker network create --driver bridge isolated_nw`
* `docker run --name rustDev -v ~/code/rustSms:/app --network=isolated_nw -p 8080:8080 -itd archrustdevenv /bin/bash`
* `docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d --network=isolated_nw postgres`
