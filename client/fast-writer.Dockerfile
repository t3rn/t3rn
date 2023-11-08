FROM node:20.1

RUN npm install -g typescript ts-node pnpm

ADD packages/fast-writer /app/fast-writer
RUN cd /app/fast-writer && pnpm install

WORKDIR /app/fast-writer

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["pnpm", "dev"]
