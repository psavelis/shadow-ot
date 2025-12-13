FROM node:18-alpine AS deps
WORKDIR /app
# Copy shared package first
COPY shared ./shared
COPY landing/package.json landing/package-lock.json* ./landing/
WORKDIR /app/landing
# Install deps (ignoring file: link errors) and manually link shared
RUN npm install --ignore-scripts || true
RUN rm -rf node_modules/@shadow-ot 2>/dev/null || true && \
    mkdir -p node_modules/@shadow-ot/shared && \
    cp -r ../shared/src node_modules/@shadow-ot/shared/src && \
    cp ../shared/package.json node_modules/@shadow-ot/shared/package.json

FROM node:18-alpine AS builder
WORKDIR /app/landing
COPY --from=deps /app/shared ../shared
COPY --from=deps /app/landing/node_modules ./node_modules
COPY landing ./
RUN npm run build

FROM node:18-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
ENV PORT=3000
EXPOSE 3000
COPY --from=builder /app/shared ./shared
COPY --from=builder /app/landing/.next ./.next
COPY --from=builder /app/landing/public ./public
COPY --from=builder /app/landing/next.config.js ./next.config.js
COPY --from=builder /app/landing/package.json ./package.json
COPY --from=builder /app/landing/node_modules ./node_modules
CMD ["npm","run","start"]
