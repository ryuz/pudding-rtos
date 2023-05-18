# Pudding-RTOS

## 概要

組込みRust で RealTime-OS 作るの楽しそうなのでやってみるというものです。

主に Xilinx社の ZynqMP にある RPU(Cortex-R5) をターゲットにしております。


## ZynqMP 環境

### クロスコンパイラ準備

```
sudo apt install gcc-arm-none-eabi
sudo apt install libnewlib-arm-none-eabi
```

### Cortex-R5 用準備

```
rustup update

rustup target add armv7r-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## サンプルプログラム

samples ディレクトリ以下にあります。

- hello  RPUを使わない通常の Linux環境下でのサンプル
- zynqmp_rpu_hello RPUでの簡単なサンプル
- zynqmp_rpu_no_kernel カーネルを使わないベアメタルのサンプル


## 姉妹品

カーネルを PL ロジックで書いてしまおうとい試みも行っております。

- https://github.com/ryuz/jelly/tree/master/projects/kv260/kv260_rtos_sample
- https://github.com/ryuz/jelly/tree/master/projects/ultra96v2/ultra96v2_rtos

