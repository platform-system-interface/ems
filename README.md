# EFI Memory Scanner

**NOTE: This utility is work in progress.**

Scan for memory data structures as known from UEFI PI firmware, i.e.,
[EDK2](https://github.com/tianocore/edk2) and derivatives.

You can access EFI memory e.g. using a Linux kernel with full access to
`/dev/mem`.

## Build

Run `make` to build a statically linked release binary.

To run the command directly with arguments, you need to explicitly pass
`--target x86_64-unknown-linux-gnu` and put arguments behind a `--`:

```sh
cargo run --release --target x86_64-unknown-linux-gnu -- -f memdump
```

## Strategy

Invoke `ems --file /dev/mem` to locate occurrences of known EFI data
structures, via their tags and also by providing a custom `--pattern`.
Use the `--offset` and `--limit` arguments to narrow down the search.
It is recommended to get a copy of that memory for offline analysis.

For example, a Lenovo ThinkPad X270's EFI memory starts at `0xb56e4000`.
That is the first address where an EFI memory "pool head" is found.
Dumping it with [u-root](https://u-root.org)'s `dd`:

```sh
dd if=/dev/mem bs=4096 skip=0xb56e4 count=43292 of=/tmp/memdump
```

The above example will dump about 190 MB.
Put the resulting file on a USB drive or copy it over network to continue.

Rerun `ems` with `--file` again, passing the path to your copy.

## Linux

You will need a kernel with specific settings to fully access `/dev/mem`.
To build your own, copy the file `linux_ems_defconfig` to your Linux tree in
the config directory as `arch/x86/configs/ems_defconfig`. For non-x86
architectures, adjust as necessary.

The configuration expects an initramfs. Pick your own or get one from
<https://github.com/linuxboot/u-root-builder> as you like. Add the `ems` command
to your custom initramfs or load it through your preferred mechanism later.

Build the kernel with the defconfig:

```sh
make ems_defconfig
make -j8
```

The resulting `arch/x86/boot/bzImage` is a PE32 binary that you can put on a FAT
partition on a GPT partitioned USB drive at `EFI/BOOT/BOOTX64.EFI`.

## TODO

- [ ] pass the `--base` to resolve references/links
- [ ] reconstruct the memory to access the data
