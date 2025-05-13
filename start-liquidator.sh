#!/bin/bash

# Only load .env if SKIP_ENV_LOAD is not set to true
if [ "$SKIP_ENV_LOAD" != "true" ]; then
    if [ -f .env ]; then
        source .env
    fi
fi
LOG_LEVEL="${RUST_LOG:=info}"

RUST_LOG="$LOG_LEVEL" ./target/release/uniswapx-artemis \
    --wss "$WS_RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    --bid-percentage "$BID_PERCENTAGE" \
    --order-type "$ORDER_TYPE" \
    --chain-id "$CHAIN_ID" \
    --executor-address "$EXECUTOR_ADDRESS"