![screenshot](./screenshot/chatbox.png)

[中文文档](./README.zh-CN.md)

#### Introduction
A chatbot based on the OpenAI chatgpt-3.5-turbo API. The program is written using `Slint-UI` and `Rust`.

#### Features
- [x] Supports `OpenAI chatgpt-3.5-turbo 4k and 16k` and `Azure OpenAI chatgpt-35-turbo 4k`
- [x] Create, save, and delete sessions
- [x] Configure a `system prompt` for each session
- [x] Supports deleting a single session question, clearing the current session, and stopping the current session
- [x] Supports configuring whether to enable historical session mode
- [x] Supports `Socks5` proxy configuration
- [x] Supports English and Chinese interface
- [x] Supports bilingual text-to-speech
- [x] Supports conversation archiving
- [x] Supports voice recording to text input
- [x] Supports concurrent output for different sessions
- [x] Supports shortcut commands to switch sessions and send questions

#### How to build?
- Install `Rust` and `Cargo`
- Run `make build`
- Refer to [Makefile](./Makefile) for more information

#### Reference
- [Slint Language Documentation](https://slint-ui.com/releases/1.0.0/docs/slint/)
- [github/slint-ui](https://github.com/slint-ui/slint)
- [Viewer for Slint](https://github.com/slint-ui/slint/tree/master/tools/viewer)
- [LSP (Language Server Protocol) Server for Slint](https://github.com/slint-ui/slint/tree/master/tools/lsp)
- [azure text2speech](https://learn.microsoft.com/zh-cn/azure/cognitive-services/speech-service/text-to-speech)
- [speech-synthesis-markup-voice](https://learn.microsoft.com/zh-cn/azure/cognitive-services/speech-service/speech-synthesis-markup-voice)
- [rest-speech-to-text-short](https://learn.microsoft.com/zh-cn/azure/cognitive-services/speech-service/rest-speech-to-text-short)
- [Azure OpenAI Server Document](https://learn.microsoft.com/zh-cn/azure/cognitive-services/openai/)
