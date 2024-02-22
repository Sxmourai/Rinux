pub use booter::*;
include!("args.rs");
fn main() {
    let args = Args::parse();
    let mut build_command = "cargo b --target x86_64-unknown-none --bin kernel ".to_string();
    if args.no_screen {
        build_command.push_str("--features no_screen ")
    }
    cmd!(panic="Failed to compile kernel",dir="kernel", "{}", build_command);
    let _ = std::fs::remove_file("target/raw.img"); // Don't unwrap because it's not mandatory to delete the file, + if the first time executing, raw.img doesn't exist
    cmd!(
        panic="Failed creating empty disk image",
        dir="target",
        "dd if=/dev/zero bs=1M count=0 seek=64 of=raw.img",
    );
    cmd!(
        panic="Failed creating MBR on disk image",
        dir="target",
        "sgdisk raw.img -n 1:2048 -t 1:ef00",
    );
    // cmd!(
    //     panic="Failed creating main partition",
    //     dir="target",
    //     "parted raw.img mkpart primary 512B 100% --script",
    // );
    cmd!(
        panic="Failed installing bios on disk with limine !",
        dir="target",
        "./limine/limine bios-install raw.img",
    );
    cmd!(
        panic="Failed formatting disk to FAT",
        dir="target",
        "mformat -i raw.img@@1M",
    );
    cmd!(
        panic="Failed creating directories on FAT device",
        dir="target",
        "mmd -i raw.img@@1M ::/EFI ::/EFI/BOOT",
    );
    cmd!(
        dir=".",
        "rm ./target/kernel",
    );
    cmd!(
        panic="Failed renaming rinux to kernel to boot from it in limine",
        dir=".",
        "mv kernel/target/x86_64-unknown-none/{}/kernel ./target/kernel",PROFILE.as_path(),
    );
    cmd!(
        panic="Failed copying files to FAT devices",
        dir="target",
        "mcopy -i raw.img@@1M kernel ../booter/limine.cfg limine/limine-bios.sys ::/"
    );
    cmd!(
        panic="Failed copying files to FAT devices",
        dir="target",
        "mcopy -i raw.img@@1M limine/BOOTX64.EFI limine/BOOTIA32.EFI ::/EFI/BOOT",
    );
    let mut qemu = "qemu-system-x86_64 -M q35 -m 2G -drive file=target/raw.img,format=raw -device isa-debug-exit,iobase=0xf4,iosize=0x04 ".to_string();
    if args.no_screen {
        qemu.push_str("-serial stdio ");
    } else {
        qemu.push_str("-serial stdio ");

    }
    if args.bios {
        cmd!("{}", qemu);
    } else { // !bios = uefi
        cmd!("{}-bios {}", qemu, ovmf_prebuilt::ovmf_pure_efi().to_string_lossy());
    }
}
