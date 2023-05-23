#!/bin/bash

curl -v -N --keepalive $AZURE_OPENAI_ENDPOINT/openai/deployments/gpt-35-turbo/chat/completions?api-version=2023-05-15 \
  -H "Content-Type: application/json" \
  -H "api-key: $AZURE_OPENAI_KEY" \
  -d '{
    "messages": [{"role": "user", "content": "给我一段Rust代码"}],
    "temperature": 0.5,
    "max_tokens": 1024,
    "stream": true,
    "top_p": 1,
    "n": 1
  }'

