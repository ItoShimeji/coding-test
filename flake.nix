{
  description = "AtCoder Rust practice environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            inherit system;
            pkgs = import nixpkgs { inherit system; };
          }
        );
    in
    {
      devShells = forAllSystems (
        { pkgs, ... }:
        {
          default = pkgs.mkShell {
            name = "atcoder-rust-practice";

            packages = [
              pkgs.online-judge-tools
            ];

            shellHook = ''
              export PATH="$PWD/scripts:$HOME/.cargo/bin:$PATH"
            '';
          };
        }
      );
    };
}
