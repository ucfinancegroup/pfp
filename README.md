# pfp

## Getting Started

Install docker.io, docker-compose, rustup.

For Development, in the root of the repo run:

```
$ sudo docker-compose -f docker-compose-dev.yml up --build
```

This builds dev containers. The rust container watches for changes and does not need to be rebuild while up.

To clean up the docker stuff when you're done:

```
$ sudo docker-compose -f docker-compose-dev.yml down
```
