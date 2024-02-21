include!("src/lib.rs");

fn main() {
    //TODO Don't have to write them all cuz we only want to exclude target
    println!("cargo:rerun-if-changed=kernel/src");
    println!("cargo:rerun-if-changed=kernel/linker.ld");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");
    println!("cargo:rerun-if-changed=kernel/.cargo"); 
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs"); 
    cmd!(panic="Failed to compile kernel",dir="../kernel","cargo b --target x86_64-unknown-none --bin kernel");
    if let Err(e) = std::fs::read_dir("../target/limine") {
        let exit_status = match e.kind() {
            std::io::ErrorKind::NotFound => cmd!(dir="../target", "git clone https://github.com/limine-bootloader/limine.git --branch=binary --depth=1"),
            _ => {
                println!("cargo:warn=\"Error whilst trying to read target/limine, trying to clone bootloader to nevertheless\"");
                cmd!(dir="../target", "git clone https://github.com/limine-bootloader/limine.git --branch=binary --depth=1")
            },
        };
        if !exit_status.success() {
            panic!("Failed fetching limine bootloader !")
        }
        cmd!(
            panic="Failed building limine !",
            dir="target/limine",
            "make",
        );
    };
}