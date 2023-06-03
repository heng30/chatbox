#!/bin/bash
proxychains curl -v -N --keepalive https://api.openai.com/v1/chat/completions \
# curl -v -N --keepalive https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "Hi"}],
    "temperature": 0.5,
    "max_tokens": 1024,
    "stream": true,
    "top_p": 1,
    "n": 1
  }'

