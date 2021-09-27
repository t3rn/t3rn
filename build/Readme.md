# Build

## Starting the environment
`./run.sh all` will do the following
- Start geth node in dev mode as docker container
- Build and start circuit
- Deploy ethereum contracts

## Cleaning up the environment
`./run.sh clean` will clean up the environment
- Stop geth node
- Stop Circuit node
- Clean up deployment cache
