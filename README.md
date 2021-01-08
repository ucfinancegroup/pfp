# pfp

## Getting Started

Install docker.io, docker-compose, rustup.

For Development, in the root of the repo run:

```
$ sudo docker-compose -f docker-compose.yml up --build
```

This builds dev containers. The rust container watches for changes and does not need to be rebuild while up.

To clean up the docker stuff when you're done:

```
$ sudo docker-compose -f docker-compose.yml down
```

## Environment Variables

For development, the server needs some configuration variables at runtime (e.g., database url). Put these in a `config.json` file inside the [server](/server) directory. Follow the format of [config.sample.json](/server/config.sample.json)

## Generating Types:

### Frontend

We need some specific options that the Swagger editor doesn't allow us to control, so we must run openapi-generator
locally. Install from: https://github.com/OpenAPITools/openapi-generator (e.g. `npm install @openapitools/openapi-generator-cli -g
`)

```npx @openapitools/openapi-generator-cli generate -g typescript-fetch -i api.yaml -o ./client/src/api -c client/apiconfig.json```
