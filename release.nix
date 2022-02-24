{ nixpkgs }:
let
  targets = import ./default.nix {
    system = "x86_64-linux";
    inherit nixpkgs;
  };
in
builtins.removeAttrs targets.mt7621 ["pkgs"]
