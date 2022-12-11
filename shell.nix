let
  sources = import ./npins;
in
{ nixpkgs ? sources.nixpkgs }:
let
  pkgs = import nixpkgs { };

  off = ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xb4e3f9fffebc1ece/set' -m '{"state": "OFF"}'
  '';
  on = ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xb4e3f9fffebc1ece/set' -m '{"state": "ON"}'
  '';

  powercycle = pkgs.writeShellScriptBin "cycle" ''
    set -ex
    ${off}
    sleep 5
    ${on}
  '';

  retry = pkgs.writeShellScriptBin "retry" ''
    PATH=${pkgs.coreutils}/bin:${pkgs.nix}/bin:${pkgs.rsync}/bin/:${pkgs.openssh}/bin
    set -ex
    ${off}
    nix-build -A mt7621.fit -o fit
    scp $(readlink -f fit) root@172.20.24.1:/var/lib/atftpd/6/C0A80101.img
    sleep 3
    ${on}
  '';

in pkgs.mkShell {
  nativeBuildInputs = [
    (pkgs.callPackage sources.npins { })
    powercycle
    retry
    pkgs.cargo
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.rustfmt
  ];
}
