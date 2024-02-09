# Use an image with Ruby installed
FROM ruby:latest

# Copy the source code into the container
COPY hello.rb /app/hello.rb

# Set the working directory
WORKDIR /app

# Run the Ruby program when the container starts
CMD ["ruby", "hello.rb"]
