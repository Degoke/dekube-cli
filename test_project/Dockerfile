FROM node:18-alpine AS builder
WORKDIR /app
COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . .

FROM node:18-alpine
WORKDIR /app
COPY --from=builder /app /app

ENV PORT=80

EXPOSE 80

CMD ["node", "index.js"]