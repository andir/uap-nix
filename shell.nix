let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  powercycle = pkgs.writeShellScriptBin "cycle" ''
    set -ex
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xcc86ecfffe8bf9a7/set' -m '{"state": "OFF"}'
    sleep 2
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xcc86ecfffe8bf9a7/set' -m '{"state": "ON"}'
  '';

  retry = pkgs.writeShellScriptBin "retry" ''
    PATH=${pkgs.coreutils}/bin:${pkgs.nix_2_3}/bin:${pkgs.rsync}/bin/:${pkgs.openssh}/bin
    set -ex
    nix-build -A mt7621.fit -o fit
    rsync -av $(readlink -f fit) root@172.20.24.1:/var/run/cudy-x6-firmware-dir/2/fit.img
    exec ${powercycle}/bin/cycle
  '';

in pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.nix_2_3
    (pkgs.callPackage sources.npins { })
    powercycle
    retry
  ];
}
