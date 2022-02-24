{ nixpkgs ? builtins.fetchTarball {
    url = "https://github.com/nixos/nixpkgs/archive/58dae9ca1c2c52990e45f358b680e8411a9dfab1.tar.gz";
  }
, system ? builtins.currentSystem
}:
let
  crossSystems = {
    ath79 = {
      crossSystem = {
        libc = "musl";
        config = "mips-unknown-linux-musl";
        openssl.system = "linux-generic32";
        withTLS = true;

        name = "ath79"; # idk
        linux-kernel = {
          name = "ath79";
          target = "uImage";
          installTarget = "uImage";
          autoModules = false;
          baseConfig = "ath79_defconfig";
        };
      };
      openwrtPatchDirectories = [
        "target/linux/generic/backport-5.10"
        "target/linux/generic/pending-5.10"
        "target/linux/ath79/patches-5.10"
      ];
      openwrtPatchFiles = [
        "target/linux/generic/files/"
        "target/linux/ath79/files/"
      ];
      structuredKernelExtraConfig = pkgs: with pkgs.lib.kernel; {
        MAGIC_SYSRQ = yes;
        MIPS_RAW_APPENDED_DTB = yes;
        DEVTMPFS = yes;
        TMPFS = yes;

        # Debugging
        IKCONFIG = yes;
        IKCONFIG_PROC = yes;

        SPI_AR934X = yes;

        # Ethernet
        AG71XX = yes;
        #GENERIC_PHY = yes;
        #GENERIC_PINCONF = yes;
        PINCTRL_SINGLE = yes;
        AT803X_PHY = yes;
        REGULATOR = yes;

        #MDIO_GPIO = yes;
        #MDIO_I2C = yes;
        MFD_SYSCON = yes;

        # WiFi
        PCI = yes;
        PCI_AR724X = yes;
        CFG80211 = yes;
        MAC80211 = yes;
        ATH_COMMON = yes;
        ATH10K = yes;
        ATH10K_PCI = yes;
        ATH10K_DEBUG = yes;

        # Other
        IPV6 = yes;

        # minimalisation
        ATH9K = no;
        RTW88 = no;
        MODULES = yes;
      };

    };

    mt7621 = {
      crossSystem = {
        libc = "musl";
        config = "mips-unknown-linux-musl";
        openssl.system = "linux-generic32";
        withTLS = true;

        name = "mt7621"; # idk
        linux-kernel = {
          name = "mt7621";
          target = "uImage";
          installTarget = "uImage";
          autoModules = false;
          baseConfig = "defconfig";
        };
      };
      openwrtPatchDirectories = [
        "target/linux/generic/backport-5.10"
        "target/linux/generic/pending-5.10"
        "target/linux/ramips/patches-5.10"
      ];
      openwrtPatchFiles = [
        "target/linux/generic/files/"
        "target/linux/ramips/files/"
      ];
      structuredKernelExtraConfig = pkgs: with pkgs.lib.kernel; {
        #CONFIG_GPIO_MT7621 = yes;
        #CONFIG_I2C_MT7621 = yes;
        #CONFIG_MT7621_WDT = yes;
        #CONFIG_MTD_NAND_MT7621 = yes;
        #CONFIG_PCI_MT7621 = yes;
        #CONFIG_PCI_MT7621_PHY = yes;
        #CONFIG_SOC_MT7621 = yes;
        #CONFIG_SPI_MT7621 = yes;



        ARCH_32BIT_OFF_T = yes;
        ARCH_HIBERNATION_POSSIBLE = yes;
        ARCH_MMAP_RND_BITS_MAX = freeform "15";
        ARCH_MMAP_RND_COMPAT_BITS_MAX = freeform "15";
        ARCH_NEEDS_CPU_IDLE_COUPLED = yes;
        ARCH_SUSPEND_POSSIBLE = yes;
        AT803X_PHY = yes;
        BLK_MQ_PCI = yes;
        BOARD_SCACHE = yes;
        BOUNCE = yes;
        CEVT_R4K = yes;
        CLKDEV_LOOKUP = yes;
        CLKSRC_MIPS_GIC = yes;
        CLOCKSOURCE_WATCHDOG = yes;
        CLONE_BACKWARDS = yes;
        #CMDLINE="rootfstype=squashfs,jffs2";
        #CMDLINE_BOOL=yes;
        # CMDLINE_OVERRIDE is not set
        COMMON_CLK = yes;
        # COMMON_CLK_BOSTON is not set
        COMPAT_32BIT_TIME = yes;
        CPU_GENERIC_DUMP_TLB = yes;
        CPU_HAS_DIEI = yes;
        CPU_HAS_PREFETCH = yes;
        CPU_HAS_RIXI = yes;
        CPU_HAS_SYNC = yes;
        CPU_IDLE = yes;
        # CPU_IDLE_GOV_LADDER is not set
        CPU_IDLE_GOV_TEO = yes;
        CPU_MIPS32 = yes;
        # CPU_MIPS32_R1 is not set
        CPU_MIPS32_R2 = yes;
        CPU_MIPSR2 = yes;
        CPU_MIPSR2_IRQ_EI = yes;
        CPU_MIPSR2_IRQ_VI = yes;
        CPU_NEEDS_NO_SMARTMIPS_OR_MICROMIPS = yes;
        CPU_PM = yes;
        CPU_R4K_CACHE_TLB = yes;
        CPU_RMAP = yes;
        CPU_SUPPORTS_32BIT_KERNEL = yes;
        CPU_SUPPORTS_HIGHMEM = yes;
        CPU_SUPPORTS_MSA = yes;
        CRC16 = yes;
        CRYPTO = yes;
        CRYPTO_ACOMP2 = yes;
        CRYPTO_DEFLATE = yes;
        #CRYPTO_HASH_INFO = yes;
        CRYPTO_LIB_POLY1305_RSIZE = freeform "2";
        CRYPTO_LZO = yes;
        CRYPTO_RNG2 = yes;
        CSRC_R4K = yes;
        DEBUG_PINCTRL = yes;
        DIMLIB = yes;
        DMA_NONCOHERENT = yes;
        # DTB_GNUBEE1 is not set
        # DTB_GNUBEE2 is not set
        DTB_RT_NONE = yes;
        DTC = yes;
        EARLY_PRINTK = yes;
        FIXED_PHY = yes;
        #FW_LOADER_PAGED_BUF = yes;
        GENERIC_ATOMIC64 = yes;
        GENERIC_CLOCKEVENTS = yes;
        GENERIC_CLOCKEVENTS_BROADCAST = yes;
        GENERIC_CMOS_UPDATE = yes;
        GENERIC_CPU_AUTOPROBE = yes;
        GENERIC_GETTIMEOFDAY = yes;
        GENERIC_IOMAP = yes;
        GENERIC_IRQ_CHIP = yes;
        GENERIC_IRQ_EFFECTIVE_AFF_MASK = yes;
        GENERIC_IRQ_SHOW = yes;
        GENERIC_LIB_ASHLDI3 = yes;
        GENERIC_LIB_ASHRDI3 = yes;
        GENERIC_LIB_CMPDI2 = yes;
        GENERIC_LIB_LSHRDI3 = yes;
        GENERIC_LIB_UCMPDI2 = yes;
        GENERIC_PCI_IOMAP = yes;
        GENERIC_PHY = yes;
        GENERIC_PINCONF = yes;
        GENERIC_SCHED_CLOCK = yes;
        GENERIC_SMP_IDLE_THREAD = yes;
        GENERIC_TIME_VSYSCALL = yes;
        GLOB = yes;
        GPIOLIB = yes;
        GPIOLIB_IRQCHIP = yes;
        GPIO_GENERIC = yes;
        GPIO_MT7621 = yes;
        # GPIO_RALINK is not set
        WATCHDOG = yes;
        GPIO_WATCHDOG = yes;
        # GPIO_WATCHDOG_ARCH_INITCALL is not set
        GRO_CELLS = yes;
        HANDLE_DOMAIN_IRQ = yes;
        HARDWARE_WATCHPOINTS = yes;
        HAS_DMA = yes;
        HAS_IOMEM = yes;
        HAS_IOPORT_MAP = yes;
        HIGHMEM = yes;
        I2C = yes;
        I2C_BOARDINFO = yes;
        I2C_CHARDEV = yes;
        I2C_GPIO = yes;
        I2C_MT7621 = yes;
        #INITRAMFS_SOURCE="";
        IRQCHIP = yes;
        IRQ_DOMAIN = yes;
        IRQ_DOMAIN_HIERARCHY = yes;
        IRQ_FORCED_THREADING = yes;
        IRQ_MIPS_CPU = yes;
        IRQ_WORK = yes;
        # KERNEL_ZSTD is not set
        NETDEVICES = yes;
        LEDS_TRIGGERS = yes;
        LED_TRIGGER_PHY = yes;
        LIBFDT = yes;
        LOCK_DEBUGGING_SUPPORT = yes;
        LZO_COMPRESS = yes;
        LZO_DECOMPRESS = yes;
        MDIO_BUS = yes;
        MDIO_DEVICE = yes;
        MEDIATEK_GE_PHY = yes;
        MEMFD_CREATE = yes;
        MFD_SYSCON = yes;
        MIGRATION = yes;
        MIKROTIK = yes;
        MIKROTIK_RB_SYSFS = yes;
        MIPS = yes;
        #MIPS_ASID_BITS=8;
        #MIPS_ASID_SHIFT=0;
        #MIPS_CBPF_JIT = yes;
        MIPS_CLOCK_VSYSCALL = yes;
        MIPS_CM = yes;
        # MIPS_CMDLINE_BUILTIN_EXTEND is not set
        # MIPS_CMDLINE_DTB_EXTEND is not set
        # MIPS_CMDLINE_FROM_BOOTLOADER is not set
        MIPS_CMDLINE_FROM_DTB = yes;
        MIPS_CPC = yes;
        MIPS_CPS = yes;
        MIPS_CPS_CPUIDLE = yes;
        # MIPS_CPS_NS16550_BOOL is not set
        MIPS_CPS_PM = yes;
        MIPS_CPU_SCACHE = yes;
        # MIPS_ELF_APPENDED_DTB is not set
        MIPS_GIC = yes;
        MIPS_L1_CACHE_SHIFT = freeform "5";
        MIPS_LD_CAN_LINK_VDSO = yes;
        MIPS_MT = yes;
        MIPS_MT_FPAFF = yes;
        MIPS_MT_SMP = yes;
        # MIPS_NO_APPENDED_DTB is not set
        #MIPS_NR_CPU_NR_MAP=4;
        MIPS_PERF_SHARED_TC_COUNTERS = yes;
        MIPS_RAW_APPENDED_DTB = yes;
        MIPS_SPRAM = yes;
        MODULES_USE_ELF_REL = yes;
        MT7621_WDT = yes;
        # MTD_CFI_INTELEXT is not set
        MTD_CMDLINE_PARTS = yes;
        MTD_NAND_CORE = yes;
        MTD_NAND_ECC = yes;
        MTD_NAND_ECC_SW_HAMMING = yes;
        MTD_NAND_MT7621 = yes;
        #MTD_NAND_MTK_BMT = yes;
        MTD_PHYSMAP = yes;
        MTD_RAW_NAND = yes;
        MTD_ROUTERBOOT_PARTS = yes;
        MTD_SPI_NOR = yes;
        #MTD_SPI_NOR_USE_VARIABLE_ERASE = yes;
        MTD_SPLIT_FIT_FW = yes;
        MTD_SPLIT_MINOR_FW = yes;
        MTD_SPLIT_SEAMA_FW = yes;
        MTD_SPLIT_TPLINK_FW = yes;
        MTD_SPLIT_TRX_FW = yes;
        MTD_SPLIT_UIMAGE_FW = yes;
        MTD_UBI = yes;
        #MTD_UBI_BEB_LIMIT=20;
        MTD_UBI_BLOCK = yes;
        #MTD_UBI_WL_THRESHOLD=4096;
        MTD_VIRT_CONCAT = yes;
        # MTK_HSDMA is not set
        NEED_DMA_MAP_STATE = yes;
        NET_DEVLINK = yes;
        NET_DSA = yes;
        NET_DSA_MT7530 = yes;
        NET_DSA_TAG_MTK = yes;
        NET_FLOW_LIMIT = yes;
        NET_MEDIATEK_SOC = yes;
        NET_SWITCHDEV = yes;
        NET_VENDOR_MEDIATEK = yes;
        # NET_VENDOR_RALINK is not set
        NO_HZ_COMMON = yes;
        NO_HZ_IDLE = yes;
        #NR_CPUS=4;
        NVMEM = yes;
        OF = yes;
        OF_ADDRESS = yes;
        OF_EARLY_FLATTREE = yes;
        OF_FLATTREE = yes;
        OF_GPIO = yes;
        OF_IRQ = yes;
        OF_KOBJ = yes;
        OF_MDIO = yes;
        OF_NET = yes;
        #PADATA = yes;
        PCI = yes;
        PCI_DISABLE_COMMON_QUIRKS = yes;
        PCI_DOMAINS = yes;
        PCI_DOMAINS_GENERIC = yes;
        PCI_DRIVERS_GENERIC = yes;
        PCI_MT7621 = yes;
        PCI_MT7621_PHY = yes;
        PERF_USE_VMALLOC = yes;
        PGTABLE_LEVELS = freeform "2";
        PHYLIB = yes;
        PHYLINK = yes;
        # PHY_RALINK_USB is not set
        PINCTRL = yes;
        PINCTRL_AW9523 = yes;
        PINCTRL_RT2880 = yes;
        # PINCTRL_SINGLE is not set
        PINCTRL_SX150X = yes;
        POWER_RESET = yes;
        POWER_RESET_GPIO = yes;
        POWER_SUPPLY = yes;
        QUEUED_RWLOCKS = yes;
        QUEUED_SPINLOCKS = yes;
        RALINK = yes;
        # RALINK_WDT is not set
        RATIONAL = yes;
        REGMAP = yes;
        REGMAP_MMIO = yes;
        REGULATOR = yes;
        REGULATOR_FIXED_VOLTAGE = yes;
        RESET_CONTROLLER = yes;
        RFS_ACCEL = yes;
        RPS = yes;
        #RTC = yes;
        RTC_CLASS = yes;
        RTC_DRV_BQ32K = yes;
        RTC_DRV_PCF8563 = yes;
        RTC_I2C_AND_SPI = yes;
        #RTC_MC146818_LIB = yes;
        SCHED_SMT = yes;
        SERIAL_8250_NR_UARTS = freeform "3";
        SERIAL_8250_RUNTIME_UARTS = freeform "3";
        SERIAL_MCTRL_GPIO = yes;
        SERIAL_OF_PLATFORM = yes;
        SGL_ALLOC = yes;
        SMP = yes;
        SMP_UP = yes;
        SOC_BUS = yes;
        SOC_MT7621 = yes;
        SPI = yes;
        SPI_MASTER = yes;
        SPI_MEM = yes;
        SPI_MT7621 = yes;
        SRCU = yes;
        SWPHY = yes;
        SYNC_R4K = yes;
        SYSCTL_EXCEPTION_TRACE = yes;
        SYS_HAS_CPU_MIPS32_R1 = yes;
        SYS_HAS_CPU_MIPS32_R2 = yes;
        SYS_HAS_EARLY_PRINTK = yes;
        SYS_SUPPORTS_32BIT_KERNEL = yes;
        SYS_SUPPORTS_ARBIT_HZ = yes;
        SYS_SUPPORTS_HIGHMEM = yes;
        SYS_SUPPORTS_HOTPLUG_CPU = yes;
        SYS_SUPPORTS_LITTLE_ENDIAN = yes;
        SYS_SUPPORTS_MIPS16 = yes;
        SYS_SUPPORTS_MIPS_CPS = yes;
        SYS_SUPPORTS_MULTITHREADING = yes;
        SYS_SUPPORTS_SCHED_SMT = yes;
        SYS_SUPPORTS_SMP = yes;
        SYS_SUPPORTS_ZBOOT = yes;
        TARGET_ISA_REV = freeform "2";
        TICK_CPU_ACCOUNTING = yes;
        TIMER_OF = yes;
        TIMER_PROBE = yes;
        TREE_RCU = yes;
        TREE_SRCU = yes;
        #UBIFS_FS = yes;
        USB_SUPPORT = yes;
        USE_OF = yes;
        WATCHDOG_CORE = yes;
        WEAK_ORDERING = yes;
        #WEAK_REORDERING_BEYOND_LLSC = yes;
        XPS = yes;
        ZLIB_DEFLATE = yes;
        ZLIB_INFLATE = yes;
      };
    };
  };

  pkgsForCrossSystem = targetSystem: import nixpkgs {
    inherit system;
    inherit (targetSystem) crossSystem;
    config.allowUnsupportedSystem = true;
    overlays = [
      (self: super:
        let inherit (self) lib; in
        {
          lib = super.lib // {
            elementsInDir = dir: lib.mapAttrsToList (name: type: { inherit type name; path = dir + "/${name}"; }) (builtins.readDir dir);
            filesInDir = dir: map ({ path, ... }: path) (super.lib.filter (entry: entry.type == "regular") (lib.elementsInDir dir));
          };
        })
    ] ++ (targetSystem.overlays or [ ]);
  };

  mkTargets = targetSystem: pkgs:
    let inherit (pkgs) lib; in
    lib.makeScope pkgs.newScope (self: {
      openwrt-src =
        let
          rev = "cbfce9236754700a343632fff8e035acbc1b1384";
          base = pkgs.pkgsBuildHost.fetchurl {
            name = "openwrt-${rev}.tar.gz";
            url = "https://git.openwrt.org/?p=openwrt/openwrt.git;a=snapshot;h=${rev};sf=tgz";
            sha256 = "08fhmw7p81l6kw1j4qbx68irh3xzsynjw5bc8rvns5wavz9irm0r";
          };
          mt7621_cudy_pr_diff = pkgs.pkgsBuildHost.fetchurl {
            url = "https://github.com/alessioprescenzo/openwrt/commit/e8b2e491d458ed6c7ac576a997a1bc6181d75106.patch";
            sha256 = "16qwazf83vd13fjvnj7z3i98svv1ix1mrs30ax5yb11kdfpyb1hy";
          };
        in
        pkgs.pkgsBuildHost.applyPatches {
          name = "openwrt-src-patched-for-cudy-x6";
          src = base;
          patches = [
            mt7621_cudy_pr_diff
          ];
        };
      uboot = (pkgs.buildUBoot {
        version = "blocktrron-uboot-cudy-x6";
        src = pkgs.pkgsBuildHost.fetchFromGitHub {
          owner = "blocktrron";
          repo = "uboot-cudy-x6";
          rev = "792865c6b352851bb4a4e3cd59da6fb456505c8c";
          sha256 = "1v9nfn5l1648904w41qjbq3cmgn9ajsa7fpgrj8wxys23k6rfjy2";
        };
        defconfig = "mt7621_cudy_x6_second_stage_defconfig";
        filesToInstall = [
          "u-boot.bin"
          "u-boot.img"
          "u-boot-lzma.img"
          "u-boot-mt7621.bin"
          "u-boot-nodtb.bin"
          "u-boot-spl-with-tpl.bin"
          "u-boot.srec"
          "u-boot.sym"
          "u-boot.cfg"
          "u-boot.cfg.configs"
        ];
      }).overrideAttrs ({ nativeBuildInputs, ... }: {
        patches = [ ];
        nativeBuildInputs = nativeBuildInputs ++ [ pkgs.pkgsBuildHost.python2 ];
      });

      initramfs = pkgs.makeInitrd {
        compressor = "${pkgs.pkgsBuildHost.zstd}/bin/zstd";
        makeUInitrd = true;
        contents = [{
          object = (pkgs.buildEnv {
            name = "uap-nix-bin";
            paths = [
              pkgs.busybox
              pkgs.hostapd
              pkgs.dropbear
              pkgs.iputils
              pkgs.tcpdump
              #(pkgs.writeScriptBin "debug-wifi" ''
              #  #!/bin/sh
              #  echo 0xffffffff > /sys/module/ath10k_core/parameters/debug_mask
              #  echo 8 > /proc/sys/kernel/printk
              #  cd /sys/bus/pci/drivers/ath10k_pci
              #  echo 0000:00:00.0 > unbind
              #  echo 0000:00:00.0 > bind
              #'')
              (lib.hiPrio (pkgs.writeScriptBin "reboot" ''
                #!/bin/sh
                echo b > /proc/sysrq-trigger
              ''))
            ];
            pathsToLink = [ "/bin" ];
          }) + "/bin";
          symlink = "/bin";
        }
          {
            object = pkgs.writeScript "init" ''
              #!/bin/sh
              set -x
              mount -t devtmpfs none /dev
              mount -t proc proc /proc
              mount -t sysfs sys /sys
              mkdir -p /run
              mount -t tmpfs tmpfs /run
              ip l set eth0 up
              #${self.cal-wifi}
              exec sh
            '';
            symlink = "/init";
          }
          {
            object = pkgs.runCommandNoCC "firmware-ath10k" { } ''
              mkdir -p $out/mediatek
              cp -r ${pkgs.firmwareLinuxNonfree}/lib/firmware/mediatek/mt7915_{rom_patch,wa,wm}.bin $out/mediatek/
              cp -r ${pkgs.wireless-regdb}/lib/firmware/regulatory.db $out/
            '';
            symlink = "/lib/firmware";
          }
          {
            object = pkgs.writeText "hostapd.conf" ''
              interface=wlan0
              ssid=test
              country_code=DE
              wpa=1
              wpa_psk=d27c89adbd8dd3f811a09bb662e78441a4842517486af5b9a4b377f460fd9fc7
              wpa_pairwise=CCMP
              hw_mode=a

              wmm_enabled=1
              ieee80211n=1
              ieee80211ac=1

              channel=0
            '';
            symlink = "/etc/hostapd.conf";
          }];
      };

      cal-wifi = pkgs.writeScript "cal-wifi" ''
        #!/bin/sh
        mtd=$(grep '"art"' /proc/mtd | cut -d : -f 1)
        dd if=/dev/$mtd of=/lib/firmware/ath10k/cal-pci-0000:00:00.0.bin iflag=skip_bytes,fullblock bs=$((0x844)) skip=$((0x5000)) count=1
      '';


      kernelSrc = (pkgs.applyPatches {
        inherit (pkgs.linux_5_10) src;
        patches = map (dir: lib.filesInDir "${self.openwrt-src}/${dir}")
          (targetSystem.openwrtPatchDirectories or [ ])
        ;
      }).overrideAttrs (o: {
        prePatch = ''
          (
          ${lib.concatMapStringsSep "\n" (dir: "${pkgs.pkgsBuildHost.rsync}/bin/rsync -rt ${self.openwrt-src}/${dir} ./") (targetSystem.openwrtPatchFiles or [])}
          )
        '';
      });

      kernel = (pkgs.buildLinux {
        inherit (pkgs.linux_5_10) version;
        src = self.kernelSrc;
        useCommonConfig = false;
        autoModules = false;
        ignoreConfigErrors = false;
        kernelPatches = [ ];
        structuredExtraConfig = pkgs.lib.mkForce ((targetSystem.structuredKernelExtraConfig or (_: { })) pkgs);
      }).overrideAttrs (o: rec {
        installPhase = ''
          mkdir -p $out
          cp -v arch/mips/boot/uImage $out/
          cp -v arch/mips/boot/vmlinu[xz]* $out/
          cp -v vmlinux $out/
        '';
      });

      dtb = pkgs.runCommandCC "uaclite.dtb" { nativeBuildInputs = [ pkgs.pkgsBuildHost.dtc ]; } ''
        unpackFile ${self.kernel.src}
        $CC -E -nostdinc -x assembler-with-cpp -I linux*/include ${self.openwrt-src}/target/linux/ramips/dts/mt7621_cudy_x6.dts -o - | dtc -o $out
      '';

      boot = pkgs.runCommandCC "boot"
        {
          nativeBuildInputs = [
            pkgs.pkgsBuildHost.ubootTools
            pkgs.pkgsBuildHost.pigz
          ];
        } ''
        set -x
        PS4=' $ '
        mkdir -p $out
        cd $out

        cat ${self.kernel}/vmlinux.bin ${self.dtb} > vmlinux.bin
        pigz -9 vmlinux.bin

        mkimage \
          -A mips \
          -O linux \
          -C gzip \
          -T kernel \
          -a 0x80001000 \
          -n Linux-${self.kernel.version} \
          -d vmlinux.bin.gz \
          $out/kernel.img

        ln -s ${self.initramfs}/initrd.img initramfs.img

        ls -lh $(readlink -f initramfs.img kernel.img)
        set +x
      '';
    });

  pkgs = pkgsForCrossSystem crossSystems.ath79;
  targets = mkTargets pkgs;
  mkDevice = crossSystem:
    let
      pkgs = pkgsForCrossSystem crossSystem;
      targets = mkTargets crossSystem pkgs;
    in
    {
      inherit (targets)
        openwrt-src
        initramfs
        cal-wifi
        kernelSrc
        kernel
        dtb
        boot
        uboot
        ;
      inherit pkgs;
    };
in
builtins.mapAttrs (name: mkDevice) crossSystems
