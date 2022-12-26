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
      structuredKernelExtraConfig = import ./kernel-config.nix;
    };
  };

  pkgsForCrossSystem = targetSystem: import nixpkgs {
    inherit system;
    inherit (targetSystem) crossSystem;
    config.allowUnsupportedSystem = true;
    overlays = [
      (import ./userspace.nix)
    ] ++ (targetSystem.overlays or [ ]);
  };

  mkTargets = targetSystem: pkgs:
    let inherit (pkgs) lib; in
    lib.makeScope pkgs.newScope (self: {
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
        compressor = "xz";
        makeUInitrd = false;
        contents = [{
          object = (pkgs.buildEnv {
            name = "uap-nix-bin";
            paths = [
              pkgs.busybox
              pkgs.hostapd
              pkgs.iproute2
              #pkgs.dropbear
              pkgs.iputils
              pkgs.tcpdump
              pkgs.iw
              pkgs.userspace
              (lib.hiPrio (pkgs.writeScriptBin "reboot" ''
                #!/bin/sh
                echo b > /proc/sysrq-trigger
              ''))
              (pkgs.writeScriptBin "mount-base" ''
                mount -t devtmpfs none /dev
                mount -t proc proc /proc
                mount -t sysfs sys /sys
                mkdir -p /run
                mount -t tmpfs tmpfs /run
                mount -t tmpfs tmpfs /tmp
                mount -t debugfs debugfs /sys/kernel/debug
              '')
            ];
            pathsToLink = [ "/bin" ];
          }) + "/bin";
          symlink = "/bin";
        }
          {
            object = pkgs.pkgsBuildHost.verifyNetconfConfig (pkgs.writeText "config.json" (builtins.toJSON {
              network.interfaces = {
                wan = {
                  oper_state = "Up";
                  accept_ra = false;
                };
                lan = {
                  oper_state = "Up";
                  link_config.Bridge = {
                    vlan_filtering = true;
                  };
                };
                lab0 = {
                  oper_state = "Up";
                  link_config.Bridge = {};
                };
              };
            }));
            symlink = "/config.json";
          }
          {
            object = pkgs.writeScript "init" ''
              #!/bin/sh
              set -x
              ls -la /bin/
              RUST_BACKTRACE=full exec initd init
              #ip l set wan up
              exec sh
            '';
            symlink = "/init";
          }
          {
            object = pkgs.runCommandNoCC "firmware-mediatek" { } ''
              mkdir -p $out/mediatek
              cp -r ${pkgs.firmwareLinuxNonfree}/lib/firmware/mediatek/mt7915_{rom_patch,wa,wm}.bin $out/mediatek/
              cp ${./mtd-backup/factory.slim} $out/mediatek/mt7915_eeprom_dbdc.bin
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

      kernelSrc =
        pkgs.linux_latest.src;

      kernel = (pkgs.buildLinux {
        inherit (pkgs.linux_latest) version;
        src = self.kernelSrc;
        useCommonConfig = false;
        autoModules = false;
        ignoreConfigErrors = false;
        modDirVersion = "6.1.1";
        kernelPatches = [
          { name = "of-fdt-fix-memblock"; patch = ./0001-of-fdt-return-1-if-early_init_dt_scan_memory-found-m.patch; }
          #{ name = "add-debug-logging"; patch = ./0001-Add-debug-logging.patch; }
          { name = "add-mtd-driver"; patch = ./0001-mtd-rawnand-add-driver-support-for-MT7621-nand-flash.patch; }
          { name = "ralink-gpio"; patch = ./802-GPIO-MIPS-ralink-add-gpio-driver-for-ralink-SoC.patch; }
          # { name = "825-i2c-MIPS-adds-ralink-I2C-driver.patch"; patch = ./825-i2c-MIPS-adds-ralink-I2C-driver.patch; }
        ];
        structuredExtraConfig = pkgs.lib.mkForce ((targetSystem.structuredKernelExtraConfig or (_: { })) pkgs);
      }).overrideAttrs (o: rec {
        postInstall = ''
          cp arch/mips/boot/vmlinux.bin $out
          cp arch/mips/boot/vmlinux.bin.gz $out
          cp arch/mips/boot/uImage $out
        '' + (lib.replaceStrings [ "find . -type f -perm -u=w -print0 | xargs -0 -r rm" ] [ "" ] o.postInstall);
      });

      dtb = pkgs.runCommandCC "cudy_x6.dtb"
        {
          nativeBuildInputs = [ pkgs.pkgsBuildHost.dtc ];
          kernel = self.kernel.dev + "/lib/modules/${self.kernel.modDirVersion}/source/";
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
        initramfs
        kernelSrc
        kernel
        dtb
        boot
        uboot
        uboot-mtd
        fit
        ;
        inherit pkgs;
        inherit (pkgs) userspace configTool;
    };
in
builtins.mapAttrs (name: mkDevice) crossSystems
