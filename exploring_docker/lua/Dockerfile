# Use an image with Lua installed
FROM ubuntu:latest

# Install Lua
RUN apt-get update && apt-get install -y lua5.3

# Copy the Lua script into the container
COPY hello.lua /app/hello.lua

# Set the working directory
WORKDIR /app

# Run the Lua script when the container starts
CMD ["lua5.3", "hello.lua"]
