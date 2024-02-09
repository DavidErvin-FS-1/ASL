# Use an image with PHP installed
FROM php:latest

# Copy the PHP script into the container
COPY hello.php /app/hello.php

# Set the working directory
WORKDIR /app

# Run the PHP script when the container starts
CMD ["php", "hello.php"]
