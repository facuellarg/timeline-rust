# docker build -t timeline .  --network host
# docker run timeline                                             

# Use the official Rust image as the build stage
FROM public.ecr.aws/docker/library/rust:1.87.0 AS timeline_build


#Copy simulator files
COPY . .

# Build the application in release mode for AAMD
RUN cargo clean && cargo build --release 

# Change to a lighter image
# Use a minimal base image to reduce the size of the final image
FROM public.ecr.aws/ubuntu/ubuntu:22.04 AS timeline_runtime

# # Install required dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the build stage
COPY --from=timeline_build /target/release/timelines /usr/local/bin/timeline

EXPOSE 6002 
# Set the startup command to run the binary
CMD ["timeline"]