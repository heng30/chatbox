#!/bin/bash

SPEECH_REGION="your_region"
SPEECH_KEY="api_key "

curl -v --location --request POST "https://${SPEECH_REGION}.tts.speech.microsoft.com/cognitiveservices/v1" \
--header "Ocp-Apim-Subscription-Key: ${SPEECH_KEY}" \
--header "Content-Type: application/ssml+xml" \
--header "X-Microsoft-OutputFormat: audio-16khz-128kbitrate-mono-mp3" \
--header "User-Agent: curl" \
--data-raw "<speak version='1.0' xml:lang='en-US'> \
    <voice xml:lang='en-US' xml:gender='Female' name='en-US-JennyNeural'> \
        my voice is my passport verify me \
    </voice> \
</speak>" > output-en.mp3

curl -v --location --request POST "https://${SPEECH_REGION}.tts.speech.microsoft.com/cognitiveservices/v1" \
--header "Ocp-Apim-Subscription-Key: ${SPEECH_KEY}" \
--header "Content-Type: application/ssml+xml" \
--header "X-Microsoft-OutputFormat: audio-16khz-128kbitrate-mono-mp3" \
--header "User-Agent: curl" \
--data-raw "<speak version='1.0' xml:lang='en-US'> \
    <voice xml:lang='zh-cn' xml:gender='Female'
        name='zh-CN-XiaomoNeural'> \
        我的声音是证明我的通信证。\
    </voice> \
</speak>" > output-cn.mp3

curl -v --location --request POST "https://${SPEECH_REGION}.tts.speech.microsoft.com/cognitiveservices/v1" \
--header "Ocp-Apim-Subscription-Key: ${SPEECH_KEY}" \
--header "Content-Type: application/ssml+xml" \
--header "X-Microsoft-OutputFormat: audio-16khz-128kbitrate-mono-mp3" \
--header "User-Agent: curl" \
--data-raw "<speak version='1.0' xml:lang='en-US'> \
    <voice name='en-US-JennyMultilingualNeural'> \
        <lang xml:lang='en-US'> \
        my voice is my passport verify me. \
        </lang> \
        <lang xml:lang='zh-CN'> \
            我的声音是证明我的通信证。\
        </lang> \
    </voice> \
</speak>" > output-multi.mp3
