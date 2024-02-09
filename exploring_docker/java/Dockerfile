# Use an image with Java installed
FROM openjdk:latest

# Copy the Java source code into the container
COPY hello.java /app/HelloWorld.java

# Set the working directory
WORKDIR /app

# Compile the Java program
RUN javac HelloWorld.java

# Run the program when the container starts
CMD ["java", "HelloWorld"]
