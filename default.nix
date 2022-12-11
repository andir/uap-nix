let
  sources = import ./npins;
in
{ nixpkgs ? sources.nixpkgs
, system ? builtins.currentSystem
}:
let
  crossSystems = {
    mt7621 = {
      crossSystem = {
        libc = "musl";
        config = "mipsel-unknown-linux-musl";
        gcc.tune = "1004kc";
        gcc.arch = "mips32r2";
        gcc.float = "soft";
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
        #"package/kernel/mac80211/patches/ath"
        #"package/kernel/mac80211/patches/ath10k"
        #"package/kernel/mac80211/patches/ath5k"
        #"package/kernel/mac80211/patches/ath9k"
        #"package/kernel/mac80211/patches/brcm"
        #"package/kernel/mac80211/patches/build"
        #"package/kernel/mac80211/patches/mwl"
        #"package/kernel/mac80211/patches/rt2x00"
        #"package/kernel/mac80211/patches/subsys"
        #"target/linux/generic/backport-5.10"
        #"target/linux/generic/pending-5.10"
        #"target/linux/ramips/patches-5.10"
      ];
      ignoredPatches = [
       # "841-USB-serial-option-add-ZTE-MF286D-modem.patch" # contained in 5.10.101
      ];
      openwrtPatchFiles = [
        "target/linux/generic/files/"
        "target/linux/ramips/files/"
      ];
      structuredKernelExtraConfig = pkgs: with pkgs.lib.kernel; {
        MODULES = yes;
        WIRELESS = yes;
        CFG80211 = yes;
        MAC80211 = yes;
        MT76_CORE = yes;
        MT7915E = yes;
        IPV6 = yes;
        WIREGUARD = module;
        WLAN_VENDOR_MEDIATEK = yes;
        
        #MODULE_SIG = yes;
        #MODULE_SIG_FORMAT = yes;
        #SYSTEM_DATA_VERIFICATION = yes; # required for mac80211

        #CONFIG_GPIO_MT7621 = yes;
        #CONFIG_I2C_MT7621 = yes;
        #CONFIG_MT7621_WDT = yes;
        #CONFIG_MTD_NAND_MT7621 = yes;
        #CONFIG_PCI_MT7621 = yes;
        #CONFIG_PCI_MT7621_PHY = yes;
        #CONFIG_SOC_MT7621 = yes;
        #CONFIG_SPI_MT7621 = yes;
        BLK_DEV_INITRD = yes;
        RD_GZIP = yes;


        ARCH_KEEP_MEMBLOCK = yes;
        ARCH_32BIT_OFF_T = yes;
        #ARCH_HIBERNATION_POSSIBLE = yes;
        ARCH_MMAP_RND_BITS_MAX = freeform "15";
        ARCH_MMAP_RND_COMPAT_BITS_MAX = freeform "15";
        #ARCH_NEEDS_CPU_IDLE_COUPLED = yes;
        #ARCH_SUSPEND_POSSIBLE = yes;
        AT803X_PHY = yes;
        BLK_MQ_PCI = yes;
        #BOARD_SCACHE = yes;
        #BOUNCE = yes;
        CEVT_R4K = yes;
        HAVE_CLK = yes;
        #CLKSRC_MIPS_GIC = yes;
        #CLOCKSOURCE_WATCHDOG = yes;
        CLONE_BACKWARDS = yes;
        #CMDLINE="rootfstype=squashfs,jffs2";
        CMDLINE = freeform "earlycon=uart8250,mmio32,0x1e000c00 earlyprintk debug";
        CMDLINE_BOOL = yes;
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
        ## CPU_IDLE_GOV_LADDER is not set
        CPU_IDLE_GOV_TEO = yes;
        CPU_MIPS32 = yes;
        ## CPU_MIPS32_R1 is not set
        CPU_MIPS32_R2 = yes;
        CPU_MIPSR2 = yes;
        #CPU_MIPSR2_IRQ_EI = yes;
        #CPU_MIPSR2_IRQ_VI = yes;
        CPU_NEEDS_NO_SMARTMIPS_OR_MICROMIPS = yes;
        CPU_PM = yes;
        CPU_R4K_CACHE_TLB = yes;
        #CPU_RMAP = yes;
        CPU_SUPPORTS_32BIT_KERNEL = yes;
        CPU_SUPPORTS_HIGHMEM = yes;
        CPU_SUPPORTS_MSA = yes;
        CRC16 = yes;
        CRYPTO = yes;
        CRYPTO_ACOMP2 = yes;
        CRYPTO_DEFLATE = yes;
        ##CRYPTO_HASH_INFO = yes;
        CRYPTO_LIB_POLY1305_RSIZE = freeform "2";
        CRYPTO_LZO = yes;
        CRYPTO_RNG2 = yes;
        CSRC_R4K = yes;
        DEBUG_PINCTRL = yes;
        #DIMLIB = yes;
        DMA_NONCOHERENT = yes;
        # DTB_GNUBEE1 is not set
        # DTB_GNUBEE2 is not set
        #DTB_RT_NONE = yes;
        DTC = yes;
        EARLY_PRINTK = yes;
        FIXED_PHY = yes;
        FW_LOADER = yes;
        FW_LOADER_COMPRESS = yes;
        #FW_LOADER_PAGED_BUF = yes;
        GENERIC_ATOMIC64 = yes;
        GENERIC_CLOCKEVENTS = yes;
        #GENERIC_CLOCKEVENTS_BROADCAST = yes;
        GENERIC_CMOS_UPDATE = yes;
        GENERIC_CPU_AUTOPROBE = yes;
        GENERIC_GETTIMEOFDAY = yes;
        GENERIC_IOMAP = yes;
        GENERIC_IRQ_CHIP = yes;
        #GENERIC_IRQ_EFFECTIVE_AFF_MASK = yes;
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
        GPIO_RALINK = yes;
        WATCHDOG = yes;
        GPIO_WATCHDOG = yes;
        # GPIO_WATCHDOG_ARCH_INITCALL is not set
        GRO_CELLS = yes;
        #HANDLE_DOMAIN_IRQ = yes;
        #HARDWARE_WATCHPOINTS = yes;
        HAS_DMA = yes;
        HAS_IOMEM = yes;
        HAS_IOPORT_MAP = yes;
        #HIGHMEM = yes;
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
        MIPS = yes;
        #MIPS_ASID_BITS=8;
        #MIPS_ASID_SHIFT=0;
        #MIPS_CBPF_JIT = yes;
        MIPS_CLOCK_VSYSCALL = yes;
        #MIPS_CM = yes;
        # MIPS_CMDLINE_BUILTIN_EXTEND is not set
        # MIPS_CMDLINE_DTB_EXTEND is not set
        # MIPS_CMDLINE_FROM_BOOTLOADER is not set
        MIPS_CMDLINE_FROM_DTB = yes;
        #MIPS_CPC = yes;
        #MIPS_CPS = yes;
        #MIPS_CPS_CPUIDLE = yes;
        # MIPS_CPS_NS16550_BOOL is not set
        #MIPS_CPS_PM = yes;
        MIPS_CPU_SCACHE = yes;
        # MIPS_ELF_APPENDED_DTB is not set
        #MIPS_GIC = yes;
        #MIPS_L1_CACHE_SHIFT = freeform "5";
        MIPS_LD_CAN_LINK_VDSO = yes;
        #MIPS_MT = yes;
        #MIPS_MT_FPAFF = yes;
        #MIPS_MT_SMP = yes;
        # MIPS_NO_APPENDED_DTB is not set
        #MIPS_NR_CPU_NR_MAP=4;
        #MIPS_PERF_SHARED_TC_COUNTERS = yes;
        MIPS_RAW_APPENDED_DTB = yes;
        MIPS_SPRAM = yes;
        #MODULES_USE_ELF_REL = yes;
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
        #MTD_ROUTERBOOT_PARTS = yes;
        MTD_SPI_NOR = yes;
        #MTD_SPI_NOR_USE_VARIABLE_ERASE = yes;
        # MTD_SPLIT_FIT_FW = yes;
        # MTD_SPLIT_MINOR_FW = yes;
        # MTD_SPLIT_SEAMA_FW = yes;
        # MTD_SPLIT_TPLINK_FW = yes;
        # MTD_SPLIT_TRX_FW = yes;
        # MTD_SPLIT_UIMAGE_FW = yes;
        MTD_UBI = yes;
        #MTD_UBI_BEB_LIMIT=20;
        MTD_UBI_BLOCK = yes;
        #MTD_UBI_WL_THRESHOLD=4096;
        #MTD_VIRT_CONCAT = yes;
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
        MEMTEST = no;
        OF = yes;
        OF_ADDRESS = yes;
        OF_EARLY_FLATTREE = yes;
        OF_FLATTREE = yes;
        OF_GPIO = yes;
        OF_IRQ = yes;
        OF_KOBJ = yes;
        OF_MDIO = yes;
        #OF_NET = yes; # included in OF
        #PADATA = yes;
        PCI = yes;
        #PCI_DISABLE_COMMON_QUIRKS = yes;
        PCI_DOMAINS = yes;
        PCI_DOMAINS_GENERIC = yes;
        PCI_DRIVERS_GENERIC = yes;
        PCIE_MT7621 = yes;
        PHY_MT7621_PCI = yes;
        PERF_USE_VMALLOC = yes;
        PGTABLE_LEVELS = freeform "2";
        PHYLIB = yes;
        PHYLINK = yes;
        # PHY_RALINK_USB is not set
        PINCTRL = yes;
        PINCTRL_RALINK = yes;
        PINCTRL_MT7621 = yes;
        #PINCTRL_AW9523 = yes;
        #SOC_RT288X = yes;
        #PINCTRL_RT2880 = yes;
        # PINCTRL_SINGLE is not set
        #PINCTRL_SX150X = yes;
        #POWER_RESET = yes;
        #POWER_RESET_GPIO = yes;
        #POWER_SUPPLY = yes;
        #QUEUED_RWLOCKS = yes;
        #QUEUED_SPINLOCKS = yes;
        RALINK = yes;
        RALINK_WDT = yes;
        RATIONAL = yes;
        #REGMAP = yes;
        #REGMAP_MMIO = yes;
        REGULATOR = yes;
        #REGULATOR_FIXED_VOLTAGE = yes;
        #RESET_CONTROLLER = yes;
        #RFS_ACCEL = yes;
        #RPS = yes;
        #RTC = yes;
        #RTC_CLASS = yes;
        #RTC_DRV_BQ32K = yes;
        #RTC_DRV_PCF8563 = yes;
        #RTC_I2C_AND_SPI = yes;
        #RTC_MC146818_LIB = yes;
        SCHED_SMT = yes;
        SERIAL_8250_NR_UARTS = freeform "3";
        SERIAL_8250_RUNTIME_UARTS = freeform "3";
        #SERIAL_8250_RT288X = yes;
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
        #SYSCTL_EXCEPTION_TRACE = yes;
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
            elementsInDir = ignores: dir: lib.mapAttrsToList (name: type: { inherit type name; path = dir + "/${name}"; })
              (lib.filterAttrs (name: value: (builtins.elem name ignores) != true)
                (builtins.readDir dir));
            filesInDir = ignores: dir: map ({ path, ... }: path) (super.lib.filter (entry: entry.type == "regular") (lib.elementsInDir ignores dir));
          };
        })
    ] ++ (targetSystem.overlays or [ ]);
  };

  mkTargets = targetSystem: pkgs:
    let inherit (pkgs) lib; in
    lib.makeScope pkgs.newScope (self: {
      #openwrt-src =
      #  let
      #    rev = "cbfce9236754700a343632fff8e035acbc1b1384";
      #    base = pkgs.pkgsBuildHost.fetchurl {
      #      name = "openwrt-${rev}.tar.gz";
      #      url = "https://git.openwrt.org/?p=openwrt/openwrt.git;a=snapshot;h=${rev};sf=tgz";
      #      sha256 = "08fhmw7p81l6kw1j4qbx68irh3xzsynjw5bc8rvns5wavz9irm0r";
      #    };
      #    mt7621_cudy_pr_diff = pkgs.pkgsBuildHost.fetchurl {
      #      url = "https://github.com/alessioprescenzo/openwrt/commit/e8b2e491d458ed6c7ac576a997a1bc6181d75106.patch";
      #      sha256 = "16qwazf83vd13fjvnj7z3i98svv1ix1mrs30ax5yb11kdfpyb1hy";
      #    };
      #  in
      #  pkgs.pkgsBuildHost.applyPatches {
      #    name = "openwrt-src-patched-for-cudy-x6";
      #    src = base;
      #    patches = [
      #      mt7621_cudy_pr_diff
      #    ];
      #  };
      uboot = (pkgs.buildUBoot {
        version = "blocktrron-uboot-cudy-x6";
        src = pkgs.pkgsBuildHost.fetchFromGitHub {
          owner = "blocktrron";
          repo = "uboot-cudy-x6";
          rev = "792865c6b352851bb4a4e3cd59da6fb456505c8c";
          sha256 = "1v9nfn5l1648904w41qjbq3cmgn9ajsa7fpgrj8wxys23k6rfjy2";
        };
        defconfig = "mt7621_cudy_x6_second_stage_defconfig";
        extraConfig = ''
          CONFIG_IMAGE_FORMAT_LEGACY=y
          CONFIG_SUPPORT_RAW_INITRD=y
          CONFIG_LOGLEVEL=8
          CONFIG_TRACE=y
          CONFIG_IDENT_STRING="Cudy X6"
          CONFIG_BOOTSTAGE_REPORT=y
        '';
        #CONFIG_CMD_TRACE=y
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
        patches = [ ./0001-arch-mips-Update-initrd_start-and-initrd_end.patch ];
        nativeBuildInputs = nativeBuildInputs ++ [ pkgs.pkgsBuildHost.python2 ];
      });

      initramfs = pkgs.makeInitrd {
        compressor = "cat";
        makeUInitrd = false;
        contents = [{
          object = (pkgs.buildEnv {
            name = "uap-nix-bin";
            paths = [
              pkgs.busybox
              #pkgs.hostapd
              #pkgs.dropbear
              pkgs.iputils
              #pkgs.tcpdump
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
              mount -t debugfs debugfs /sys/kernel/debug

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
        mtd=$(grep '"firmware"' /proc/mtd | cut -d : -f 1)
        dd if=/dev/$mtd of=/lib/firmware/ath10k/cal-pci-0000:00:00.0.bin iflag=skip_bytes,fullblock bs=$((0x844)) skip=$((0x5000)) count=1
      '';


      kernelSrc =
        pkgs.linux_latest.src;

      kernel = (pkgs.buildLinux {
        inherit (pkgs.linux_latest) version;
        src = self.kernelSrc;
        useCommonConfig = false;
        autoModules = false;
        ignoreConfigErrors = false;
        kernelPatches = [
          { name = "add-debug-logging"; patch = ./0001-Add-debug-logging.patch; }
          { name = "add-mtd-driver"; patch = ./0001-mtd-rawnand-add-driver-support-for-MT7621-nand-flash.patch; }
          { name = "debug-gpiolib"; patch = ./0001-debug-gpiolib.patch; }
          { name = "ralink-gpio"; patch = ./802-GPIO-MIPS-ralink-add-gpio-driver-for-ralink-SoC.patch; }
#          { name = "825-i2c-MIPS-adds-ralink-I2C-driver.patch"; patch = ./825-i2c-MIPS-adds-ralink-I2C-driver.patch; }
        ];
        structuredExtraConfig = pkgs.lib.mkForce ((targetSystem.structuredKernelExtraConfig or (_: { })) pkgs);
      }).overrideAttrs (o: rec {
        postInstall = ''
          cp arch/mips/boot/vmlinux.bin $out
          cp arch/mips/boot/vmlinux.bin.gz $out
          cp arch/mips/boot/uImage $out
        '' + (lib.replaceStrings ["find . -type f -perm -u=w -print0 | xargs -0 -r rm"] [""] o.postInstall);
      });

      dtb = pkgs.runCommandCC "cudy_x6.dtb" {
        nativeBuildInputs = [ pkgs.pkgsBuildHost.dtc ];
        kernel = self.kernel.dev + "/lib/modules/${self.kernel.version}/source/";
        input = ./mt7621_cudy_x6.dts;
        outputs = [ "out" "yaml" "dts" ];
      } ''
        echo testing
        test -e $kernel/arch/mips/boot/dts/ralink/mt7621.dtsi || echo "mt7621.dtsi not found"
        $CC -E -nostdinc -x assembler-with-cpp -I $kernel/include -I $kernel/arch/mips/boot/dts/ralink $kernel/arch/mips/boot/dts/ralink/mt7621.dtsi -o test > /dev/null || exit 123

        echo preprocessing
        $CC -E -nostdinc -x assembler-with-cpp -I $kernel/include -I $kernel/arch/mips/boot/dts/ralink -o precompiled $input
        sed -e '/^# [0-9]/d' -i precompiled

        echo generating yaml
        dtc -O yaml -o $yaml precompiled

        echo generating dts
        dtc -O dts -o $dts precompiled

        echo generating binary 
        dtc -o $out precompiled
      '';

      # Create a uboot image that can be booted from the builtin cudy loader.
      # We can't change the name as the loader matches on that.
      uboot-mtd = pkgs.runCommandNoCC "uboot-mtd-payload"
        {
          nativeBuildInputs = [
            pkgs.pkgsBuildHost.ubootTools
          ];
          uboot = self.uboot + "/u-boot.bin";
        } ''
        mkdir $out
        set -x
        mkimage -A mips -O linux -T kernel -C none -e 0x80700000 -a 0x80700000 -n R13 -d $uboot $out/firmware-mtd.bin
        mkimage -l $out/firmware-mtd.bin
        set +x
      '';

      fit-its = pkgs.pkgsBuildHost.substituteAll {
        src = ./fit.its;
        kernel = self.kernel + "/vmlinux.bin";
        kernelAddress = "0x80001000";

        dtb = self.dtb;
        dtbAddress = "0x82000000";

        initrd = self.initramfs + "/initrd";
        initrdAddress = "0x81000000";
      };

      fit = pkgs.runCommand "fit.itb"
        {
          nativeBuildInputs = [
            pkgs.pkgsBuildHost.ubootTools
            pkgs.pkgsBuildHost.dtc
          ];
        }
        ''
          cp ${self.fit-its} fit.its
          mkimage -f fit.its $out
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
          -C none \
          -T kernel \
          -a 0x80001000 \
          -n Linux-${self.kernel.version} \
          -d vmlinux.bin.gz \
          $out/kernel.img

        ln -s ${self.initramfs}/initrd.img initramfs.img

        ls -lh $(readlink -f initramfs.img kernel.img)
        set +x
      '';

      netconf = pkgs.rustPlatform.buildRustPackage {
        name = "netconf";
        src = ./netconf;
        cargoLock = {
          lockFile = ./netconf/Cargo.lock;
        };
      };


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
        uboot-mtd
        netconf
        fit
        ;
      inherit pkgs;
    };
in
builtins.mapAttrs (name: mkDevice) crossSystems
