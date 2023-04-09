#!/bin/bash
proxychains curl -v https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "Hello!"}],
    "temperature": 0.5,
    "max_tokens": 10,
    "top_p": 1,
    "n": 1,
    "stop": "."
  }'

