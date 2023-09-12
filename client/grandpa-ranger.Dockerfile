FROM node:20.1

RUN npm install -g typescript ts-node pnpm

ADD packages/grandpa-ranger /app/grandpa-ranger
RUN cd /app/grandpa-ranger && pnpm install

WORKDIR /app/grandpa-ranger

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["pnpm", "start"]
