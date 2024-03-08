alias b := build
alias r := run
alias c := clean

default: build

clean: shallow-clean
  cargo clean

shallow-clean:
  rm -rf tmp

build mode="debug": shallow-clean
  #!/usr/bin/env sh
  set -ex

  if [[ {{mode}} == "debug" ]]; then
    mode_flag=""
  else
    echo "=== running in mode {{mode}}"
    mode_flag="--{{mode}}"
  fi

  mkdir -p tmp
  cargo build -Z unstable-options $mode_flag --out-dir tmp
  for file in asm/*.asm; do
    nasm -f elf64 $file -o tmp/$(basename $file).o
  done

  ld -n --gc-sections \
    -o image/boot/kernel.bin \
    -T asm/linker.ld \
    tmp/*.o \
    -L tmp \
    -l kernel

  grub-mkrescue -o target/rikos.iso image

run mode="debug": (build mode)
  #!/usr/bin/env sh
  qemu-system-x86_64 -drive format=raw,file=target/rikos.iso -serial stdio
