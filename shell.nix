let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  off = ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xcc86ecfffe8bf9a7/set' -m '{"state": "OFF"}'
  '';
  on = ''
    ${pkgs.mosquitto}/bin/mosquitto_pub -h 10.250.43.1 -t 'zigbee2mqtt/0xcc86ecfffe8bf9a7/set' -m '{"state": "ON"}'
  '';

  powercycle = pkgs.writeShellScriptBin "cycle" ''
    set -ex
    ${off}
    sleep 5
    ${on}
  '';

  retry = pkgs.writeShellScriptBin "retry" ''
    PATH=${pkgs.coreutils}/bin:${pkgs.nix_2_3}/bin:${pkgs.rsync}/bin/:${pkgs.openssh}/bin
    set -ex
    ${off}
    nix-build -A mt7621.fit -o fit
    scp $(readlink -f fit) root@172.20.24.1:/var/run/cudy-x6-firmware-dir/2/fit.img
    sleep 3
    ${on}
  '';

in pkgs.mkShell {
  nativeBuildInputs = [
    (pkgs.callPackage sources.npins { })
    powercycle
    retry
  ];
}
