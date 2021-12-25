# Pudding-RTOS

## 概要

組込みRust で RealTime-OS 作るの楽しそうなのでやってみるというものです。

主に Xilinx社の ZynqMP にある RPU(Cortex-R5) をターゲットにしております。

なお、現在、ZynqMP の PL(FPGA部)を使った RTOS を Jelly-RTOS として開発しております。


## ZynqMP 環境

### クロスコンパイラ準備

```
sudo apt install gcc-arm-none-eabi
sudo apt install libnewlib-arm-none-eabi
```

### Cortex-R5 用準備

```
rustup update
rustup install nightly
rustup default nightly

rustup target add armv7r-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
