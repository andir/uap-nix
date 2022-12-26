let sources = import ./npins; in
{ nixpkgs ? sources.nixpkgs }:
let
  targets = import ./default.nix {
   system = "x86_64-linux";
    inherit nixpkgs;
  };
  shellDependencies = let
    shell = import ./shell.nix { inherit nixpkgs; };
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in pkgs.runCommand "shell-deps" { shellDrv = shell.drvPath; } ''
    touch $out
  '';
in
(builtins.removeAttrs targets.mt7621 ["pkgs"])
// {
  inherit shellDependencies;
}
