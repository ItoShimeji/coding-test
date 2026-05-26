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
        let
          patchedOnlineJudgeApiClient =
            pkgs.python3Packages.online-judge-api-client.overridePythonAttrs
              (oldAttrs: {
                postPatch =
                  (oldAttrs.postPatch or "")
                  + ''
                    python - <<'PY'
                    from pathlib import Path

                    path = Path("onlinejudge/service/atcoder.py")
                    source = path.read_text()

                    source = source.replace(
                        "parsed_memory_limit = re.search(r'^(メモリ制限|Memory Limit): ([0-9.]+) (KB|MB)', memory_limit)",
                        "parsed_memory_limit = re.search(r'^(メモリ制限|Memory Limit): ([0-9.]+) (KB|KiB|MB|MiB)', memory_limit)",
                    )

                    old = (
                        "        if memory_limit_unit == 'KB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1000)\n"
                        "        elif memory_limit_unit == 'MB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1000 * 1000)\n"
                        "        else:\n"
                        "            assert False\n"
                    )
                    new = (
                        "        if memory_limit_unit == 'KB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1000)\n"
                        "        elif memory_limit_unit == 'KiB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1024)\n"
                        "        elif memory_limit_unit == 'MB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1000 * 1000)\n"
                        "        elif memory_limit_unit == 'MiB':\n"
                        "            memory_limit_byte = int(float(memory_limit_value) * 1024 * 1024)\n"
                        "        else:\n"
                        "            assert False\n"
                    )
                    source = source.replace(old, new)

                    path.write_text(source)
                    PY
                  '';
              });

          patchedOnlineJudgeTools =
            pkgs.python3Packages.online-judge-tools.override {
              online-judge-api-client = patchedOnlineJudgeApiClient;
            };
        in
        {
          default = pkgs.mkShell {
            name = "atcoder-rust-practice";

            packages = [
              patchedOnlineJudgeTools
            ];

            shellHook = ''
              repo_root="$PWD"
              while [ "$repo_root" != "/" ] && [ ! -d "$repo_root/scripts" ]; do
                repo_root="$(dirname "$repo_root")"
              done

              if [ -d "$repo_root/scripts" ]; then
                export PATH="$repo_root/scripts:$HOME/.cargo/bin:$PATH"
              else
                export PATH="$HOME/.cargo/bin:$PATH"
              fi
            '';
          };
        }
      );
    };
}
