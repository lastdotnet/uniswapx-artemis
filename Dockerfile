# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.81
ARG APP_NAME=uniswapx-artemis

################################################################################
# Create a stage for building the application.

FROM public.ecr.aws/docker/library/rust:${RUST_VERSION}-bookworm AS build
ARG APP_NAME
WORKDIR /app


# AWS CodeBuild doesn't seem to support buildkit so can't use --mount
COPY . .
RUN cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
FROM public.ecr.aws/debian/debian:bookworm-slim AS final
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
