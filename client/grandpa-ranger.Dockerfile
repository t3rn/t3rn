FROM node:20.1

RUN npm install -g typescript

ADD grandpa-ranger /app/packages/grandpa-ranger
RUN cd /app/packages/grandpa-ranger && yarn 

WORKDIR /app/packages/grandpa-ranger

# node is default user with UID 1000 in this image
RUN chown -R node /app
USER node

CMD ["yarn", "start"]
