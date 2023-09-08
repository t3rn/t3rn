FROM node:20.1

RUN apt update -y
RUN apt install -y python
RUN npm install -g typescript ts-node pnpm

ADD packages/cli /app/cli
RUN cd /app/cli && pnpm i

ADD packages/cli/entrypoint.sh /app/cli/entrypoint.sh
RUN chmod +x /app/cli/entrypoint.sh

WORKDIR /app/cli

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

ENTRYPOINT ["/app/cli/entrypoint.sh"]
