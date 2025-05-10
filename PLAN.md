# MVP Development Plan

This document outlines the tasks required to build a Minimum Viable Product (MVP) for the YouTube Comments Filter Backend Service.

## Core Tasks

1.  **Set up the Rust project:**
    *   Initialize a new Rust project using `cargo`.
    *   Add necessary dependencies (e.g., Actix-web/Rocket, reqwest, dotenv/config-rs, serde).

2.  **Implement the Config Layer:**
    *   Add `.env` support using a library like `dotenv`.
    *   Load configuration values from environment variables.
    *   Define configuration parameters for AI tokens, API ports, and environment toggles.

3.  **Implement the API server:**
    *   Create an API server using a Rust framework (Actix-web or Rocket).
    *   Define the API routes.

4.  **Implement API Contract Validation:**
    *   Define structured request/response types using `serde`.
    *   Implement input sanitization and payload validation for the `/analyze` endpoint.

5.  **Implement the cache layer (Local Memory):**
    *   Use a simple global mapping (e.g., `HashMap`) in local memory for caching.
    *   Implement cache invalidation logic (e.g., LRU).

6.  **Implement the AI module:**
    *   Integrate with DeepSeek and Gemini APIs for spam classification.
    *   Implement logic to handle API requests and responses.
    *   Stub retry/fallback logic for external AI APIs.

7.  **Create API endpoints:**
    *   Implement the `/analyze` endpoint to accept YouTube comments and return spam classification.
    *   Implement the `/health` endpoint for health checks.
    *   Implement the `/cache/:keyword` endpoint to inspect cache hits.

8.  **Implement the response format:**
    *   Ensure the API returns the response in the specified CSV format (`S,K,C`).

9.  **Implement the spam classification logic:**
    *   Use the AI models to classify comments as spam or not spam.
    *   Implement logic to normalize keywords.

10. **Write tests:**
    *   Write unit tests for individual components.
    *   Write integration tests to ensure the service is working correctly.

11. **Set up observability:**
    *   Implement tracing, metrics, and logging for monitoring the service.
    *   Choose a suitable observability platform (e.g., Prometheus, Grafana).

12. **Achieve a good MVP:**
    *   Ensure the core functionality is working correctly and the service is stable.

13. **Integrate Redis:**
    *   Integrate Redis for caching.
    *   Implement caching logic for AI analysis results.

14. **Create a Dockerfile:**
    *   Create a Dockerfile to containerize the service.
    *   Ensure the Docker image is optimized for production.

15. **Set up CI/CD Pipeline:**
    *   Set up a GitHub Actions pipeline to run `cargo test`, `cargo fmt`, and `docker build`.

16. **Implement Basic Security Practices:**
    *   Implement basic rate limiting or IP filtering for the `/analyze` endpoint.
    *   Ensure secure handling of API tokens.

17. **Write documentation:**
    *   Write documentation for the service.
    *   Include instructions on how to run the service, configure it, and contribute to it.

## Out of Scope

The following features are explicitly out of scope for the MVP:

*   AI fallback logic (using a different AI model if the primary one fails)
*   Database storage for comments and classifications
