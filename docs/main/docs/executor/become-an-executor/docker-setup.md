# T3RN Executor Setup Guide

## Overview
This guide provides instructions for setting up and running a T3RN executor using Docker.

## Prerequisites
- Docker installed on your system
- Docker Compose installed on your system
- Basic knowledge of Docker and container operations
- A valid private key for the executor

## Setup Instructions

1. Clone the official T3RN executor release repository:
   ```bash
   git clone https://github.com/t3rn/executor-release.git t3rn-executor
   cd t3rn-executor
   ```

2. The repository contains the necessary Dockerfile and docker-compose.yml files. You'll need to configure your environment by updating the docker-compose.yml file:
   - Replace the `PRIVATE_KEY_LOCAL` value with your actual private key
   - Adjust other settings as needed (gas price, logging, etc.)

3. Start the executor:
   ```bash
   docker-compose up -d
   ```

4. Check the logs:
   ```bash
   docker-compose logs -f
   ```

## Upgrade Guide

1. Check the latest version at the [T3RN Executor Releases page](https://github.com/t3rn/executor-release/releases/)

2. To upgrade your executor:
   - Open your Dockerfile
   - Locate the version line at the top: `ARG APP_VERSION=v0.31.0`
   - Change the version number to the latest release version, e.g `ARG APP_VERSION=v0.32.0`

3. Rebuild and restart your containers:
   ```bash
   # Stop the current container
   docker-compose down

   # Rebuild with the new version
   docker-compose build --no-cache

   # Start the updated container
   docker-compose up -d

   # Check logs to verify successful upgrade
   docker-compose logs -f
   ```

Note: It's recommended to always check the release notes for any breaking changes or additional configuration requirements before upgrading.

## Additional Resources
- [Gas Fee Explorer](https://brn.explorer.caldera.xyz/)
- [T3RN GitHub Repository](https://github.com/t3rn)

## Support
For additional support or questions, visit the T3RN documentation or community channels.