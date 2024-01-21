riscv64-linux-gnu-gcc app/d-app.c -o bin/d-app -lcmt
xgenext2fs -B 4096 -b 2048 -d bin /tmp/dapp.ext2