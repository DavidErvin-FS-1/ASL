# Use an image with Go installed
FROM golang:latest

# Copy the source code into the container
COPY hello.go /app/hello.go

# Set the working directory
WORKDIR /app

# Compile the Go program
RUN go build -o hello hello.go

# Run the program when the container starts
CMD ["./hello"]
