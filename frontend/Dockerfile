FROM node:18 AS base

# Install dependencies only when needed
FROM base AS deps
# Check https://github.com/nodejs/docker-node/tree/b4117f9333da4138b03a546ec926ef50a31506c3#nodealpine to understand why libc6-compat might be needed.
RUN apt install git python3 gcc make g++
WORKDIR /app

# Install dependencies based on the preferred package manager
COPY package.json yarn.lock* ./
RUN npm i --force

# Rebuild the source code only when needed
FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Next.js collects completely anonymous telemetry data about general usage.
# Learn more here: https://nextjs.org/telemetry
# The following line disables telemetry during the build.
ENV NEXT_TELEMETRY_DISABLED 1
ARG NEXT_PUBLIC_ROOT_MANAGER_ADDRESS
ARG NEXT_PUBLIC_GENERATE_ERC20_PREDICATE_ADDRESS
ARG NEXT_PUBLIC_BRIDGE_MANAGER_INDEX
ARG NEXT_PUBLIC_ETH_TOKEN_ADDRESS
ARG NEXT_PUBLIC_ETHEREUM_PROVIDER_NETWORK
ARG NEXT_PUBLIC_ETHEREUM_EXPLORER_URL
ARG NEXT_PUBLIC_NETWORK_GENESIS_BLOCK_HASH
ARG NEXT_PUBLIC_API_URL
ARG NEXT_PUBLIC_CCDSCAN_URL
ARG NEXT_PUBLIC_BRIDGE_MANAGER
ARG NEXT_PUBLIC_CIS2_BRIDGEABLE
ARG NEXT_PUBLIC_ROOT_MANAGER_DEPOSIT_OVERHEAD_GAS
ARG NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ERC20_GAS
ARG NEXT_PUBLIC_ROOT_MANAGER_WITHDRAW_ETH_GAS
ARG NEXT_PUBLIC_BRIDGE_MANAGER_WITHDRAW_ENERGY
ARG NEXT_PUBLIC_CCD_NODE_URL
ARG NEXT_PUBLIC_CCD_NODE_PORT


RUN yarn build

# Production image, copy all the files and run next
FROM base AS runner
WORKDIR /app

ENV NODE_ENV production
# The following disables telemetry during runtime.
ENV NEXT_TELEMETRY_DISABLED 1

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs

COPY --from=builder /app/public ./public

# Automatically leverage output traces to reduce image size
# https://nextjs.org/docs/advanced-features/output-file-tracing
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000

ENV PORT 3000

CMD ["node", "server.js"]
