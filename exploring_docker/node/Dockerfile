# Use an image with Node.js
FROM node:latest

# Create a directory for the application
WORKDIR /app

# Copy the source code into the container
COPY package.json .
COPY hello.js .

# Install dependencies
RUN npm install

# Run the program when the container starts
CMD ["node", "hello.js"]
