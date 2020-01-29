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

###動作確認

- ビルドして実行（serverディレクトリで行う）

```bash
cargo run
```
