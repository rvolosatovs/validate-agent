{
  nixConfig.extra-substituters = [
    "https://rvolosatovs.cachix.org"
    "https://nix-community.cachix.org"
    "https://cache.garnix.io"
  ];
  nixConfig.extra-trusted-public-keys = [
    "rvolosatovs.cachix.org-1:eRYUO4OXTSmpDFWu4wX3/X08MsP01baqGKi9GsoAmQ8="
    "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
  ];

  description = "User-Agent checking utility";

  inputs.nixify.url = github:rvolosatovs/nixify;

  outputs = {nixify, ...}:
    nixify.lib.rust.mkFlake {
      src = ./.;

      targets.wasm32-wasi = false;

      clippy.workspace = true;
      clippy.allTargets = true;

      test.workspace = true;
      test.allTargets = true;

      buildOverrides = {pkgs, ...} @ args: {nativeBuildInputs ? [], ...}: {
        nativeBuildInputs = [
          pkgs.protobuf # build dependency of prost-build v0.9.0
        ];
      };
    };
}
