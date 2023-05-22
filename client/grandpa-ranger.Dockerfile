FROM node:20.1

RUN npm install -g typescript ts-node

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/grandpa-ranger /app/grandpa-ranger
RUN cd /app/grandpa-ranger && yarn 

WORKDIR /app/grandpa-ranger

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["yarn", "start"]
