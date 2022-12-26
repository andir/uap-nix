# expressions for the userspace tools we provide
self: super:
let
  inherit (self) lib;

  cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  workspaceMembers = cargoTOML.workspace.members;

  src =
    let
      src = ./.;
      baseDir = toString src;
      memberExpressions = (map (member: "^/${member}") workspaceMembers) ++
        map (member: "^/${member}/.+") workspaceMembers
      ;
      extraExpressions = [
        "^/Cargo.toml$"
        "^/Cargo.lock$"
      ];
      expressions = memberExpressions ++ extraExpressions;
    in
    builtins.path {
      path = src;
      filter = path:
        let suffix = lib.removePrefix baseDir path; in
        _: lib.any
          (r: builtins.match r suffix != null)
          expressions;
      name = "source";
    };
in
{
  userspace = self.rustPlatform.buildRustPackage {
    name = "userspace";
    src = src;
    nativeBuildInputs = [ self.pkgsBuildHost.glibc ]; # for getconf to get syscalls
    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "wl-nl80211-0.1.0" = "sha256-Xoi9JNzL7JTA3vTkw6wD0Neeq3abFHnOw3koaIosUXU=";
      };
    };
    cargoBuildFlags = [ "-p initd -p netconf -p wirelessd" ];
  };
  configTool = self.userspace.overrideAttrs (_: {
    cargoBuildFlags = [ "-p config-tool" ];
    postInstall = "";
  });
  conf_schema = self.runCommandNoCC "schema.json "
    {
      nativeBuildInputs = [
        (self.pkgsBuildHost.configTool)
      ];
    } "config-tool generate-schema > $out";

  verifyNetconfConfig = file: self.runCommandNoCC "config.json"
    { nativeBuildInputs = [ self.python3Packages.jsonschema ]; inherit file; } ''
    jsonschema ${self.conf_schema} < $file && cp $file $out
  '';
}
