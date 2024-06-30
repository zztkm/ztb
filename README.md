# ztb

zztkm tool box.

Rust の練習がてら Linux コマンドを再実装したり、自分がほしいコマンドを実装したりしています。

## Install

```bash
git clone https://github.com/zztkm/ztb.git
cd ztb

cargo install --path .
```

## Usage


```bash
# 引数がない場合は標準入力から読み取ります
$ cat README.md | cargo run -- cat                                                                                   at 01:46:25
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/ztb cat`
# ztb

zztkm tool box.

# 引数ありの場合
$ cargo run -- cat README.md Makefile                                                                                at 01:48:46
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/ztb cat README.md Makefile`
# ztb

zztkm tool box.

.PHONY: all
all:
	cargo build

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy
```

