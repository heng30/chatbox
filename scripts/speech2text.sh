#!/bin/bash
SPEECH_REGION="your-region"
SPEECH_KEY="your-api-key"

audio_file=@'./output-en.wav'
curl -v --location --request POST \
"https://${SPEECH_REGION}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1?language=en-US&format=simple" \
--header "Ocp-Apim-Subscription-Key: ${SPEECH_KEY}" \
--header "Content-Type: audio/wav" \
--data-binary $audio_file

audio_file=@'./output-cn.wav'
curl -v --location --request POST \
"https://${SPEECH_REGION}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1?language=zh-CN&format=simple" \
--header "Ocp-Apim-Subscription-Key: ${SPEECH_KEY}" \
--header "Content-Type: audio/wav" \
--data-binary $audio_file
