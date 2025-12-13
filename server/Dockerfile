FROM node:slim

ENV NODE_ENV development

WORKDIR /server

COPY package.json package-lock.json .
COPY src src

RUN npm install

EXPOSE 5000
