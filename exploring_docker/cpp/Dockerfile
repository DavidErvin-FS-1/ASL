# Use an image with a C++ compiler
FROM gcc:latest

# Copy the source code into the container
COPY hello.cpp /app/hello.cpp

# Set the working directory
WORKDIR /app

# Compile the C++ program
RUN g++ -o hello hello.cpp

# Run the program when the container starts
CMD ["./hello"]
