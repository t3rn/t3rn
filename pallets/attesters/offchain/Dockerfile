FROM node:20-alpine

ADD . /app
WORKDIR /app

RUN npm i

ENTRYPOINT ["node", "offchain-attester"]
