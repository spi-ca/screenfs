use fractal_fuse::{MountOptions, Session};
use holefs::{CliArgs, HoleFs, RuntimeConfig};

fn main() -> std::io::Result<()> {
    let args = match CliArgs::parse_from(std::env::args_os()) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    if !args.source_root.is_dir() {
        eprintln!(
            "source root is not a directory: {}",
            args.source_root.display()
        );
        std::process::exit(2);
    }
    if !args.mount_root.is_dir() {
        eprintln!(
            "mount root is not a directory: {}",
            args.mount_root.display()
        );
        std::process::exit(2);
    }

    let cfg = RuntimeConfig::from_cli(args)
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;

    eprintln!(
        "mounting holefs: source={} mount={} readonly={} hide-policy=compiled io_uring=required",
        cfg.source_root.display(),
        cfg.mount_root.display(),
        cfg.readonly
    );
    eprintln!("FUSE_OVER_IO_URING negotiation is mandatory; session startup fails without it.");

    let opts = MountOptions::new()
        .fs_name("holefs")
        // Handler-level readonly is the source of truth. Do not set mount-level ro
        // until mount smoke proves it preserves hidden-before-EROFS semantics.
        .read_only(false)
        .force_readdir_plus(true)
        .default_permissions(false)
        .allow_other(false);

    let mount_root = cfg.mount_root.clone();
    Session::new(mount_root, opts)?.run(HoleFs::new(cfg))
}
