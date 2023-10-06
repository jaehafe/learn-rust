./target/debug/cargo_test -> Hello, World!

## cargo run

Hello, world!

## cargo check

- 코드가 컴파일되는지를 빠르게 확인해주지만 실행파일을 생성하지는 않습니다

### 요약

- cargo build나 cargo check를 사용하여 프로젝트를 빌드
- cargo run 빌드하고 실행
- Cargo는 이를 target/debug 디렉토리에 저장

## 릴리즈 빌드

`cargo build --release`

- 최적화와 함께 이를 컴파일
