FROM node:18.12

RUN npm install -g typescript ts-node

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/types /app/types
RUN cd /app/types && yarn install && yarn build

ADD packages/cli /app/cli
RUN cd /app/cli && yarn 

ADD packages/tsconfig.json /app/cli/

WORKDIR /app/cli

ENTRYPOINT ["ts-node", "index.ts"]
