FROM node:20.1

RUN npm install -g typescript ts-node

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/types /app/types
RUN cd /app/types && yarn install && yarn build

ADD packages/attester /app/attester
RUN cd /app/attester && yarn 

WORKDIR /app/attester

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["yarn", "start"]
