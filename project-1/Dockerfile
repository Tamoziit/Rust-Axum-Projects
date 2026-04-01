#stage 1
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

#stage 2
FROM debian:bookworm-slim
WORKDIR /app
#copying everything from Rust builder into debian image where we have the build release for our project inside /app
COPY --from=builder /app/target/release/project-1 .
EXPOSE 8000
CMD ["./project-1"]