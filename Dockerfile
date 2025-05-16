# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.81
ARG APP_NAME=uniswapx-artemis
# Set the target platform explicitly
ARG TARGETPLATFORM=linux/amd64


################################################################################
# Create a stage for building the application.

FROM --platform=${TARGETPLATFORM} public.ecr.aws/docker/library/rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
ARG TARGETPLATFORM
WORKDIR /app

# Install cross-compilation tools if needed
RUN if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
        rustup target add x86_64-unknown-linux-gnu; \
        echo "Building for x86_64"; \
        export TARGET="x86_64-unknown-linux-gnu"; \
    elif [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
        rustup target add aarch64-unknown-linux-gnu; \
        apt-get update && apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu; \
        echo "Building for aarch64"; \
        export TARGET="aarch64-unknown-linux-gnu"; \
    else \
        echo "Building for host architecture"; \
    fi

# AWS CodeBuild doesn't seem to support buildkit so can't use --mount
COPY . .
RUN if [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
        cargo build --locked --release --target x86_64-unknown-linux-gnu && \
        cp ./target/x86_64-unknown-linux-gnu/release/$APP_NAME /bin/server; \
    elif [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
        cargo build --locked --release --target aarch64-unknown-linux-gnu && \
        cp ./target/aarch64-unknown-linux-gnu/release/$APP_NAME /bin/server; \
    else \
        cargo build --locked --release && \
        cp ./target/release/$APP_NAME /bin/server; \
    fi

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
FROM --platform=${TARGETPLATFORM} public.ecr.aws/debian/debian:bookworm-slim AS final
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get update -y && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /app/uniswapx-artemis

# Expose the port that the application listens on.
EXPOSE 1559

# Make the binary executable
RUN chmod +x /app/uniswapx-artemis

# Change to CMD to allow override from ECS
CMD ["/app/uniswapx-artemis"]
