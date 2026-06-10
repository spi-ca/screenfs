use fractal_fuse::{MountOptions, Session};
use screenfs::{LaunchArgs, RuntimeConfig, ScreenFs};

fn main() -> std::io::Result<()> {
    let argv = std::env::args_os().collect::<Vec<_>>();
    if LaunchArgs::wants_help(argv.iter().cloned()) {
        println!("{}", LaunchArgs::help());
        std::process::exit(0);
    }

    let args = match LaunchArgs::parse_from(argv) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    if !args.cli.source_root.is_dir() {
        eprintln!(
            "source root is not a directory: {}",
            args.cli.source_root.display()
        );
        std::process::exit(2);
    }
    if !args.cli.mount_root.is_dir() {
        eprintln!(
            "mount root is not a directory: {}",
            args.cli.mount_root.display()
        );
        std::process::exit(2);
    }

    let cfg = RuntimeConfig::from_launch(args)
        .map_err(|message| std::io::Error::new(std::io::ErrorKind::InvalidInput, message))?;

    eprintln!(
        "mounting screenfs: source={} mount={} mutability-family={} mutability-source={} readonly-rules={} allow-write-rules={} hide-policy=compiled io_uring=required",
        cfg.source_root.display(),
        cfg.mount_root.display(),
        cfg.mutability_family().as_str(),
        cfg.mutability_source().as_str(),
        cfg.readonly_rule_count(),
        cfg.allow_write_rule_count()
    );

    let opts = MountOptions::new()
        .fs_name("screenfs")
        // Handler-level readonly is the source of truth. Do not set mount-level ro
        // until mount smoke proves it preserves hidden-before-EROFS semantics.
        .read_only(false)
        .force_readdir_plus(true)
        .default_permissions(false)
        .allow_other(false);

    let mount_root = cfg.mount_root.clone();
    Session::new(mount_root, opts)?.run(ScreenFs::new(cfg))
}
