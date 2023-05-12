FROM node:20.1

RUN npm install -g typescript

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/types /app/types
RUN cd /app/types && yarn install && yarn build

ADD packages/executor /app/executor
RUN cd /app/executor && yarn 

WORKDIR /app/executor

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["yarn", "start"]
