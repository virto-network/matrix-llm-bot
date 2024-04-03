# Matrix-llm-bot
Using the <a href="https://matrix-org.github.io/matrix-rust-sdk/matrix_sdk/index.html">matrix-sdk</a> for Rust to develop a bot that provides the latest news about the Kusama community.

## Usage
To implement this code for any Matrix user you own, follow these steps:
<ol>
  <li> Set your password as an environment variable in a file named .env placed in the main directory.</li>
  <li> Modify line 57 of main.rs to include your own username and homeserver.</li>
  <li> Once you have made these changes, navigate to the src directory and run <code>cargo run</code>.</li>
</ol>

## Python Service

This bot only works among with the [Python Service for Matrix LLM-Bot](https://github.com/ail3ngrimaldi/python-llm-service) which instructions you'll find there. The service listens to the bot events.