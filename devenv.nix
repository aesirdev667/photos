{ inputs, lib, pkgs, config, ... }:

let
  unstable = import inputs.nixpkgs-unstable { system = pkgs.stdenv.system; };
in

{
  cachix.enable = true;
  cachix.pull = [ "pre-commit-hooks" ];

  env.TAURI_APP_PATH = "apps/tauri/";
  env.TAURI_FRONTEND_PATH = "apps/ui/";

  languages.javascript.enable = true;
  languages.javascript.bun.enable = true;
  languages.javascript.bun.package = unstable.bun;

  languages.rust.enable = true;
  languages.rust.channel = "stable";
  languages.rust.components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "llvm-tools-preview" ];

  packages = with pkgs; [
    cargo-expand
    cargo-tauri
    cargo-watch
    sea-orm-cli
  ] ++ lib.optionals pkgs.stdenv.isLinux [
    glib
  ];

  git-hooks.hooks = {
    cargo-check.enable = true;
    check-added-large-files.enable = true;
    check-case-conflicts.enable = true;
    clippy.enable = true;
    clippy.settings.denyWarnings = true;
    rustfmt.enable = true;
    rustfmt.settings.all = true;
    rustfmt.settings.check = true;
  };

  scripts = {
    workdir.exec = ''
      # get working directory
      WORK_DIR="$(git rev-parse --show-superproject-working-tree 2>/dev/null)"
      if [ -z "$WORK_DIR" ]; then
        WORK_DIR="$(git rev-parse --show-toplevel)"
      fi
      echo "$WORK_DIR"
    '';

    nuxi.exec = ''
      (
        cd "$(workdir)/apps/ui" || { echo "Failed to cd to $(workdir)/apps/ui"; exit 1; }
        ${lib.getExe config.languages.javascript.bun.package} x nuxi "$@"
      )
    '';

    tauri.exec = ''
      (
        cd "$(workdir)" || { echo "Failed to cd to $(workdir)"; exit 1; }
        ${lib.getExe pkgs.cargo-tauri} "$@" || { echo "Failed to excute 'cargo-tauri $*'"; exit 1; }
      )
    '';

    sea-orm-cli.exec = ''
      (
        cd "$(workdir)" || { echo "Failed to cd to $(workdir)"; exit 1; }
        ${lib.getExe' pkgs.sea-orm-cli "sea-orm-cli"} "$@"
      )
    '';
  };

  tasks = {
    "photos:dbUrl" = {
      exec = ''
        app_id="$(jq -r .identifier < "$(workdir)/apps/tauri/tauri.conf.json")"
        db_name="photos.db"

        case "$(uname -s)" in
          Darwin*)
            db_path="$HOME/Library/Application Support/$app_id/$db_name"
            ;;
          Linux*)
            db_path="$HOME/.local/share/$app_id/$db_name"
            ;;
          MINGW*|CYGWIN*|MSYS*)
            db_path="$APPDATA\\$app_id\\$db_name"
            ;;
        esac

        export DATABASE_URL="sqlite://$db_path?mode=rwc"
      '';
      exports = [ "DATABASE_URL" ];
      before = [ "devenv:enterShell" ];
    };

    "photos:coverage" = {
      exec = ''
        (
          cd "$WORK_DIR" || { echo "Failed to cd to $WORK_DIR"; exit 1; }
          [ -d "./coverage" ] && rm -r "./coverage"
          mkdir ./coverage

          echo "=== running coverage ==="
          CARGO_INCREMENTAL=0 \
            RUSTFLAGS="-Cinstrument-coverage" \
            LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw" \
            ${lib.getExe' config.languages.rust.toolchain.cargo "cargo"} test
          echo "ok."

          echo "=== generating report ==="
          fmt="html"
          file="coverage/html"
          if ${if config.devenv.isTesting then "true" else "false"}; then
            fmt="lcov"
            file="coverage/tests.lcov"
          fi
          ${lib.getExe pkgs.grcov} . \
            --binary-path="./target/debug/deps" \
            -s . \
            -t "$fmt" \
            --branch \
            --ignore-not-existing \
            --ignore "../*" \
            --ignore "/*" \
            --ignore "*/tests/*" \
            --ignore "target/*" \
            -o "$file"
          echo "ok."

          echo "=== cleaning up ==="
          find -name "*.profraw" -exec rm "{}" \;
          echo "ok."

          if ${if config.devenv.isTesting then "false" else "true"}; then
            open "coverage/html/index.html" 
          fi

          echo "report location: $file"
        )
      '';

      after = [ "devenv:enterTest" ];
    };
  };

  test = pkgs.writeShellScript "devenv-test" ''
    # FIX: we don't want it to run `enterShell` twice
    # echo "• Setting up shell environment ..."
    # ${config.enterShell}

    set -euo pipefail
    echo "• Testing ..."
    ${config.enterTest}
  '';
}
