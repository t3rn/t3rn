# Attester

Attester is a TypeScript application that performs attestation tasks.

## Prerequisites

- Node.js and npm should be installed on your machine.
- yarn should be installed globally. If not, you can install it by running the following command:

  ```bash
  npm install -g yarn
  ```

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/t3rn/t3rn.git
   ```

1. Build dependencies

   ```bash
   cd client/packages
   make all
   ```

1. Install dependencies using yarn:

   ```bash
   cd client/packages/attester
   yarn install
   ```

1. Copy .envrc-example to .envrc and supply it with base64 encoded keys used for attester

1. Run it
   ```bash
   cd client/packages/attester
   yarn run start
   ```

## Configuration

1. Adjust `.envrc` file in the root of the project and add the necessary environment variables. 

1. Update the `config` files with your desired configuration.

## Usage

To start the Attester application, run the following command:

```bash
yarn start
```

The application will connect to the required clients and start listening to events for attestation.

## Scripts

- `yarn fmt`: Formats the source code using Prettier.
- `yarn lint`: Lints the source code using ESLint.
- `yarn lint:report`: Generates an ESLint report in JSON format.
- `yarn start`: Starts the Attester application.

## Issues

In case it's exitting without any apparent reason then just comment out line:

```
stderr.write = NullWritable.write.bind(NullWritable)
```
