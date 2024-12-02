# SCG Ask
https://ask.scg.skku.ac.kr/

scg cs(customer service) website, 토이프로젝트처럼 진행해 봤습니다.

정말 간단하게 폼 작성하면 scg 메일로 문의내용이 전달되는 기능만 수행합니다.

프론트엔드에서 wasm 프레임워크인 yew를 사용해 봤는데, 

배포중인 pod에 할당된 cpu가 하나여서 그런지 리소스 병렬 서빙이 안되고 있습니다..

## dev
### Backend
#### dependencies
* tokio
* actix-web

```
cd ./backend
cargo install
cargo run
```

### Frontend
#### dependencies
* yew
```
cd ./frontend
cargo install
cargo make serve
```

### commit & push
프로젝트의 루트로 나와서 커밋 및 푸시를 해주세요.
