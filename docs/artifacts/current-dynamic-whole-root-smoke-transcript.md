# Current dynamic visible-glob whole-root ScreenFS smoke transcript

- Captured: 2026-06-12T02:18:53+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs
- Source root: /
- Mount root: /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt

## Config
```yaml
visibility:
  default: hidden
  visible:
    - /etc/**/*.conf
mutability:
  default: readonly
```

## Mount
```text
$ timeout 90s target/debug/screenfs / /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt --config /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/config.yaml
mounted pid=168315 startup_ms=298
stderr:
mounting screenfs: source=/ mount=/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required
```

## Dynamic bridge-visible checks
```text
$ sh -c find '/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt' -maxdepth 2 -print | sed -n '1,40p'
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/NetworkManager
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/PackageKit
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/UPower
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/X11
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/arptables.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/audit
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/avahi
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/bluetooth
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/containers
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/default
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/e2scrub.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ebtables.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/firewalld
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fonts
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fprintd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fuse.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fwupd
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gai.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gdm
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/geoclue
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gtk-3.0
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/healthd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/host.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ipp-usb
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/kernel
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/keyd
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/kmscon
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/krb5.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ld.so.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ld.so.conf.d
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/libaudit.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/libva.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/libvirt
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/locale.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/logrotate.d
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/makepkg.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/makepkg.conf.d
$ sh -c find '/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc' -maxdepth 2 -name '*.conf' -print | sed -n '1,40p'
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/NetworkManager/NetworkManager.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/PackageKit/CommandNotFound.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/PackageKit/PackageKit.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/PackageKit/Vendor.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/UPower/UPower.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/arptables.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/audit/auditd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/avahi/avahi-daemon.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/bluetooth/input.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/bluetooth/main.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/bluetooth/network.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/containers/containers.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/containers/mounts.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/containers/registries.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/containers/storage.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/classes.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/cups-files.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/cupsd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/printers.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/snmp.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/cups/subscriptions.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/default/cpufreq-bench.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/default/cpupower-service.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/e2scrub.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ebtables.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/firewalld/firewalld.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fonts/fonts.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fprintd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fuse.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/fwupd/fwupd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gai.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gdm/custom.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/geoclue/geoclue.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/gtk-3.0/im-multipress.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/healthd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/host.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/ipp-usb/ipp-usb.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/kernel/install.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/keyd/50-local-kbd.conf
/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc/keyd/55-apple-magic-keyboard.conf
$ stat /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc
  File: /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/etc
  Size: 3268      	Blocks: 8          IO Block: 4096   directory
Device: 0,78	Inode: 2           Links: 1
Access: (0755/drwxr-xr-x)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 2026-06-11 23:35:30.592557906 +0900
Modify: 2026-06-11 13:44:02.783936198 +0900
Change: 2026-06-11 13:44:02.783936198 +0900
 Birth: -
$ stat /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/root
stat: cannot statx '/tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt/root': No such file or directory
exit_status=1
```

## Performance snapshot
```text
rss_kb=2172
fd_count=3
```

## Unmount
```text
$ fusermount3 -u /tmp/screenfs-dynamic-whole-root-smoke-NAEg2p/mnt
unmounted
```
