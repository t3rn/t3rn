FROM node:20.1

RUN npm install -g typescript ts-node

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/types /app/types
RUN cd /app/types && yarn install && yarn build

ADD packages/cli /app/cli
RUN cd /app/cli && yarn 

ADD packages/cli/tsconfig.json /app/cli/
ADD cli_entrypoint.sh /app/cli/entrypoint.sh
RUN chmod +x /app/cli/entrypoint.sh

WORKDIR /app/cli

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

ENTRYPOINT ["/app/cli/entrypoint.sh"]
