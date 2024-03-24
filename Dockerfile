# Use the cargo-lambda image for building
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder

# Create a directory for your application
WORKDIR /usr/src/app

# Copy your source code into the container
COPY . .

# Build the Lambda function using cargo-lambda
RUN cargo lambda build --release --arm64

# Use a new stage for the final image
# copy artifacts to a clean image
FROM public.ecr.aws/lambda/provided:al2-arm64

# Create a directory for your lambda function
WORKDIR /new-lambda-project

# Copy the bootstrap binary from the builder stage
COPY --from=builder /usr/src/app/target/ ./ 

# Check to make sure files are there 
RUN if [ -d /new-lambda-project/lambda/tinayiluo_ids721_week7/ ]; then echo "Directory '/new-lambda-project' exists"; else echo "Directory '/new-lambda-project' does not exist"; fi
RUN if [ -f /new-lambda-project/lambda/tinayiluo_ids721_week7/bootstrap]; then echo "File '/new-lambda-project/lambda/lambda/bootstrap' exists"; else echo "File '/new-lambda-project/lambda/lambda/bootstrap' does not exist"; fi

# Set the entrypoint for the Lambda function
ENTRYPOINT ["/new-lambda-project/lambda/tinayiluo_ids721_week7/bootstrap"]