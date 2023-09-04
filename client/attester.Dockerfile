FROM node:20.1

RUN npm i -g add pnpm typescript ts-node

ADD packages/attester /app/attester
RUN cd /app/attester && pnpm install

WORKDIR /app/attester

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["pnpm", "start"]
