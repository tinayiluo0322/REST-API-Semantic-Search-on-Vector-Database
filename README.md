[![pipeline status](https://gitlab.com/ly178/tinayiluo_ids721_week7/badges/main/pipeline.svg)](https://gitlab.com/ly178/tinayiluo_ids721_week7/-/commits/main)

# REST API Semantic Search on Vector Database

## Introduction

This project is centered around the creation and utilization of a vector database to manage and search through sports-related data based on semantic similarity. Leveraging advanced technologies like Qdrant for vector database management, Cohere for generating text embeddings, and Rust for backend development, this initiative aims to provide a seamless way to perform semantic searches. With the help of Docker and AWS services, the application is designed to be scalable, reliable, and easily deployable.

## Goals

- Data Ingestion: Efficiently ingest sports data into a vector database.
- Semantic Queries and Aggregations: Perform advanced queries and aggregations based on semantic similarity.
- Visualization: Offer an intuitive visualization of the search outputs.
- REST API: Develop a simple, robust REST API using Rust to interact with the database.
- Containerization: Containerize the service using Docker for easy deployment and scaling.
- CI/CD: Implement Continuous Integration and Continuous Deployment (CI/CD) pipelines for automated testing and deployment.

## How It Works

Upon entering a search query related to sports, the system leverages a vector database to return the top 5 sports that closely match the search terms. This process involves data ingestion into the vector database, semantic search query execution, and visualization of the search results through a simple REST API, accessible via a public endpoint.

Example Demonstration:

My query is “Play a ball”: `https://9hpt1l4en0.execute-api.us-east-1.amazonaws.com/week7_new_stage/?sports=%27Play%20a%20ball%27`

![Screen_Shot_2024-03-24_at_6.49.00_PM](/uploads/bb000b54cf89ff985ceaa8bb6a1982d3/Screen_Shot_2024-03-24_at_6.49.00_PM.png)

## Technical Overview

- Vector Database Management: Utilize Qdrant for storing and querying vector data.
- Embedding Generation: Use Cohere to convert textual data into embeddings, enhancing the semantic search capabilities.
- Rust Web Microservice: Create a robust backend service in Rust to handle API requests.
- Containerization: Employ Docker to encapsulate the service, ensuring portability and consistency across environments.
- CI/CD: Set up CI/CD pipelines for automated build, test, and deployment workflows.

## Development Stages

### Stage 1: Initialize Cargo Lambda
1. `cargo lambda new <repository name>`

### Stage 2: Sign Up for Qdrant and Cohere
1. Sign up for Qdrant, create a vector database cluster, and obtain the API key and cluster URL.
2. Sign up for Cohere and obtain an API key to generate embeddings for the vector database.

### Stage 3: Modify Rust Content
1. Store Qdrant and Cohere API keys in a `.env` file.
2. Create a `sports.jsonl` file containing keys for various sports and their descriptions.

### Mini Stage 1: Data Ingestion
1. Modify `setup.rs`: This script is a Rust-based utility designed for initializing and populating a collection within Qdrant, optimized for storing and searching through embeddings. It serves as a comprehensive tool for initializing a Qdrant collection with vector embeddings generated from text descriptions, utilizing Cohere's API.
2. Add two bins in the `cargo.toml` file. As this is a separate package, bins are temporarily added to run the file separately from `main.rs` (these are commented out later for easier deployment debugging).
3. Export Access Key:
    ```
    set -a
    source .env
    set +a
    ```
4. Run the bin for data ingestion: `cargo run --bin setup_test sports.jsonl`.
5. Open Qdrant dashboard and check the test collection. The Qdrant cluster should now have a collection filled with associated embeddings or "points".

### Mini Stage 2: Visualization and Aggregation
1. Modify `main.rs`: This script is designed to act as an AWS Lambda function that processes web requests, interfaces with the Cohere API to generate text embeddings, and uses those embeddings to perform similarity searches in a Qdrant vector database. Essentially, `main.rs` functions as a serverless service leveraging AWS Lambda to integrate text embedding capabilities from Cohere with vector search functionality from Qdrant. It dynamically handles web requests, generates embeddings for text queries, searches for similar items in a Qdrant vector database, and returns the search results as HTTP responses.
2. Use `cargo lambda watch` to test locally.
3. Test the visualization and aggregation by sending a GET request: `http://localhost:9000/?sports='Play a ball` in Postman.

### Stage 3: Set Up AWS IAM User for Lambda Function
1. Navigate to IAM, create a user, and attach policies: `IAMFullAccess`, `AmazonEC2ContainerRegistryFullAccess`, and `AWSLambda_FullAccess`.
2. Upon completing user creation, open user details, and navigate to security credentials.
3. Generate an access key (select OTHER).
4. Store `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and `AWS_REGION` (your choice) in your `.env` file (ensure this file is not pushed with your repo, add it to `.gitignore`).
5. Export the variables again.
6. Build the project for release to ensure it functions correctly: `cargo lambda build --release`.
7. Ensure the API gateway functions correctly without a Docker image for testing.
8. Deploy the project for release: `cargo lambda deploy --env-file .env`.
9. Log into AWS Lambda (ensure the correct region is selected) and verify installation.
10. Upon confirmation, connect the Lambda function with AWS API Gateway.
11. Create a new API (retain default settings, REST API), then create a new resource (the URL path to be appended to your API link) and enable the CORS option (to connect with different APIs outside of AWS).
12. Create a method to attach the lambda function (since this is an HTTP lambda function, enable lambda proxy integration).
13. Deploy the stage, then test the URL.

### Stage 4: Build and Attach an Image
1. If testing above API gateway without a Docker image is successful, proceed to create a Docker image.
2. Modify the Dockerfile: This Dockerfile outlines a multi-stage build process for creating a Docker image suitable for deploying an AWS Lambda function written in a language supported by the cargo-lambda tool, specifically targeting the ARM64 architecture.
3. Build the image via Amazon ECR. Navigate to the ECR registry and create a new private registry. Copy the login commands.
4. Start Docker and execute `docker buildx build --progress=plain --platform linux/arm64 -t week7 .`.
5. Follow the ECR guide steps to push the image to the ECR repository.
6. In Lambda, create a function using the image, then search for your image, select arm64 in the options, and proceed.
7. Since this is a new function, add your environment variables (`QDRANT_URI`, `QDRANT_KEY`, `COHERE_KEY`) in the configuration of your Lambda function.
8. Create a new API (keep default settings, REST API) then create a new resource (the URL path that will be appended to your API link) and enable the CORS option (to connect to different APIs outside of AWS).
9. Create a method to attach the lambda function (since this is an HTTP lambda function, turn on lambda proxy integration).
10. Deploy the stage, then test the URL.

### Stage 5: CI/CD Pipeline and Deployment
1. Modify the `.gitlab-ci.yml` file: The pipeline is divided into two main stages: ‘build-test’ and ‘deploy’. In the build-test stage, it sets up the development environment, installs necessary languages and tools, and performs linting, formatting, testing, and building tasks. In the deploy stage, it prepares for deployment by building a Docker image, tagging it appropriately, and pushing it to AWS ECR for deployment.
2. Attach environment variables to GitLab (`AWS_ACCESS_KEY_ID`, `AWS_REGION`, `AWS_SECRET_ACCESS_KEY`, `EC`, `ECR`).
3. Push the repository. This CI/CD pipeline ensures that code changes are automatically tested and deployed, streamlining the development workflow and facilitating continuous integration and continuous deployment practices.

## Deliveries:

### Sample Viz (Deployment)
My query is “Play a ball”: `https://9hpt1l4en0.execute-api.us-east-1.amazonaws.com/week7_new_stage/?sports=%27Play%20a%20ball%27`

![Screen_Shot_2024-03-24_at_6.49.00_PM](/uploads/bb000b54cf89ff985ceaa8bb6a1982d3/Screen_Shot_2024-03-24_at_6.49.00_PM.png)

### Qdrant Testing

![Screen_Shot_2024-03-24_at_6.48.08_PM](/uploads/4002ebcd4127005a901313c780891e91/Screen_Shot_2024-03-24_at_6.48.08_PM.png)

### Postman Testing (Local)

![Screen_Shot_2024-03-24_at_7.04.15_PM](/uploads/48c5e06052a9e8d64d2988a954e17155/Screen_Shot_2024-03-24_at_7.04.15_PM.png)

### Docker Image in ECR Container Registry

![Screen_Shot_2024-03-24_at_6.51.17_PM](/uploads/2a29103abef757da438265011a678410/Screen_Shot_2024-03-24_at_6.51.17_PM.png)

![Screen_Shot_2024-03-24_at_6.53.32_PM](/uploads/2fe0c0ea4447bf15996311ec5b3459db/Screen_Shot_2024-03-24_at_6.53.32_PM.png)

### Lambda Architecture Diagram with API Gateway

![Screen_Shot_2024-03-24_at_6.55.52_PM](/uploads/1958a7eb781bdf24a4a72b6883c09d4b/Screen_Shot_2024-03-24_at_6.55.52_PM.png)

### CICD pipeline to Push to ECR Contianer (Linked to Lambda Function)

![Screen_Shot_2024-03-24_at_6.59.30_PM](/uploads/b9ca9b2b6c6e3539814b5fd0fdb02bd0/Screen_Shot_2024-03-24_at_6.59.30_PM.png)

## Conclusion

This project exemplifies the power of combining vector databases, machine learning embeddings, and modern web technologies to create a semantic search engine. By utilizing tools like Rust, Docker, AWS Lambda, and CI/CD pipelines, I’ve built a scalable and efficient system that simplifies the process of searching for sports-related information based on semantic similarity.