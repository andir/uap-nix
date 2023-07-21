let
  sources = import ./npins;
in
{ nixpkgs ? sources.nixpkgs }:
let
  pkgs = import nixpkgs { };

  off = pkgs.writeShellScriptBin "off" ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xb4e3f9fffebc1ece/set' -m '{"state": "OFF"}'
  '';
  on = pkgs.writeShellScriptBin "on" ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xb4e3f9fffebc1ece/set' -m '{"state": "ON"}'
  '';

  powercycle = pkgs.writeShellScriptBin "cycle" ''
    set -ex
    ${off}/bin/off
    sleep 5
    ${on}/bin/on
  '';

  retry = pkgs.writeShellScriptBin "retry" ''
    PATH=${pkgs.coreutils}/bin:${pkgs.nix}/bin:${pkgs.rsync}/bin/:${pkgs.openssh}/bin
    set -ex
    ${off}/bin/off
    WORKDIR=${toString ./.}
    nix-build $WORKDIR -A mt7621.fit -o $WORKDIR/fit
    rsync -vzP $(readlink -f $WORKDIR/fit) root@172.20.24.1:/var/lib/atftpd/6/C0A80101.img
    sleep 3
    ${on}/bin/on
  '';

in pkgs.mkShell {
  nativeBuildInputs = [
    (pkgs.callPackage sources.npins { })
    #off
    #on
    powercycle
    retry
    off
    on
    pkgs.cargo
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.rustfmt
    pkgs.capnproto
  ];
}
