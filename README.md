# IKN-attend-system
在席確認システム

# 環境構築
## server
### rustの導入

- これをターミナルに入力

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- PATHを通す
```bash
source $HOME/.cargo/env
```

### 必要なバイナリーファイルをインストール

- これをserverディレクトリ上で実行

```bash
cargo install systemfd cargo-watch
```

### 動作確認

- ビルドして実行（serverディレクトリで行う）

```bash
cargo run
```

- api確認
 ```bash
 curl -X {Request} http://localhost:8080/api/{query}/{arg}
 ```
 
 ### API

 最低限これだけあればよし
 #### PUT
 - [x]`api/attend/<id>`
 json {
   "id": 3,
   "username": "obata",
   "role": "B4",
   "state": 1,
 }
 - [x]`api/leave/<id>`
 json {
   "id": 4,
   "username": "mata",
   "role": "B4",
   "state": 0,
 }
 
 #### GET
 - [x]`api/students/`
 json
 ```
 [
  {
   "id": 1,
   "username": "monkukui",
   "role": "B4",
   "state": 0,
  },
  {
   "id": 2,
   "username": "rossy",
   "role": "B4",
   "state": 1,
  },
 ]
 ```
 - [x] `api/student/<id>`
 json
 ```
 {
  "id": 2,
  "username": rossy,
  "role": "B4",
  "state": 1,
 }
 ```

### POST
- [ ] api/register/
 To:
 json

 {
  "username":
  "role":
 }

#### state対応表
  
  0. Leave
  0. Attend

## misc

### masterブランチでのcommitを禁止する

 - .git/hooks/pre-commit に以下を記述

 ```:pre-commit

  #!/bin/sh

  # if the branch is master, then fail.

  branch="$(git symbolic-ref HEAD 2>/dev/null)" || \
    "$(git describe --contains --all HEAD)"

  if [ "${branch##refs/heads/}" = "master" ]; then
    printf "\e[31m%s\n\e[m" "[Error]"
    echo "can't commit on master branch."
    echo "please commit on topic branch."
    exit 1
  fi
  ```

 - 実行権限を与えておく（ターミナルで入力）
 ```bash
 chmod a+x .git/hoooks/pre-commit
 ```
