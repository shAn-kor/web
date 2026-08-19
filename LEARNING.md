# Rust Web Learning

이 저장소는 Rust 웹 백엔드를 단계적으로 학습하기 위한 프로젝트다.

## 현재 단계

- [x] Rust 2024 edition 프로젝트
- [x] Axum HTTP 서버
- [x] Tokio async runtime
- [x] tracing 기반 로그
- [x] `/` 기본 라우트
- [x] `/health` 헬스 체크
- [ ] JSON request/response
- [ ] Path / Query extractor
- [ ] AppState 공유
- [ ] 에러 타입과 `IntoResponse`
- [ ] PostgreSQL + SQLx
- [ ] migration / transaction
- [ ] middleware
- [ ] integration test
- [ ] graceful shutdown

## 실행

```bash
cargo run
```

서버는 기본적으로 `127.0.0.1:3000`에서 실행된다.

```bash
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/health
```

로그 레벨을 바꾸려면 `RUST_LOG`를 사용한다.

```bash
RUST_LOG=debug cargo run
```

## 학습 원칙

초기에는 하나의 crate 안에서 Rust의 ownership, error handling, async/await와 Axum의 extractor/router/state 구조를 익힌다.

Spring 스타일의 Controller/Service/Repository 계층을 처음부터 기계적으로 복제하거나, 필요하기 전에 trait 기반 DI/DDD/workspace 구조를 도입하지 않는다. 기능이 늘어나면서 경계가 실제로 필요해질 때 모듈을 분리한다.

## 다음 목표

다음 단계는 작은 `User` API를 만들어 다음 흐름을 한 번에 익히는 것이다.

1. `POST /users` JSON body 받기
2. `GET /users/{id}` path parameter 받기
3. `serde`로 request/response 타입 정의하기
4. 메모리 상태를 `State`로 공유하기
5. 에러를 HTTP status와 response로 변환하기

그 다음 PostgreSQL과 SQLx를 연결한다.
