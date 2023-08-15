FROM node:20.2

RUN npm i -g typescript pnpm

ADD packages/sdk /app/sdk
RUN cd /app/sdk && yarn install && yarn build

ADD packages/types /app/types
RUN cd /app/types && yarn install && yarn build

ADD packages/executor /app/executor
RUN cd /app/executor && pnpm i 

WORKDIR /app/executor

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["pnpm", "run", "start"]
