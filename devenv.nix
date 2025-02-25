{ pkgs, ... }:
{
  env.TAURI_APP_PATH = "apps/tauri/";
  env.TAURI_FRONTEND_PATH = "apps/ui/";

  languages.javascript.enable = true;
  languages.javascript.bun.enable = true;
  languages.javascript.bun.package = pkgs.unstable.bun;
  languages.rust.enable = true;
  languages.rust.channel = "beta";

  packages = with pkgs; [ cargo-expand cargo-tauri sea-orm-cli ];

  scripts.tauri.exec = ''
    (
      # change working directory to git toplevel
      cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)"
      # execute command from here
      ${pkgs.lib.getExe pkgs.cargo-tauri} "$@"
    )
  '';

  scripts.nuxi.exec = ''
    (
      # change working directory to git toplevel/apps/ui
      cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)/apps/ui"
      # execute command from here
      ${pkgs.lib.getExe pkgs.unstable.bun} x nuxi "$@"
    )
  '';

  scripts."sea-orm-cli".exec = ''
    ROOT="$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)/apps/tauri"
    APP_ID=$(cat "$ROOT/tauri.conf.json" | ${pkgs.lib.getExe pkgs.jq} -r .identifier)
    DB_NAME=photos.db

    case "$(uname -s)" in
      Darwin*)
        DB_PATH="$HOME/Library/Application Support/$APP_ID/$DB_NAME"
        ;;
      Linux*)
        DB_PATH="$HOME/.local/share/$APP_ID/$DB_NAME"
        ;;
      MINGW*|CYGWIN*|MSYS*)
        DB_PATH="$APPDATA\\$APP_ID\\$DB_NAME"
        ;;
    esac

    DATABASE_URL="sqlite://$DB_PATH?mode=rwc" ${pkgs.lib.getExe pkgs.sea-orm-cli} "$@"
  '';

  scripts.update-submodules.exec = ''
    (
      # change working directory to git toplevel
      cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)"
      # update and apply patch to xtaskops
      ${pkgs.lib.getExe pkgs.git} submodule update --remote vendors/xtaskops
      cd "vendors/xtaskops"
      ${pkgs.lib.getExe pkgs.git} apply ../patches/xtaskops_ci_args.patch
    )
  '';

  # TODO: add a script to generate code coverage
  # scripts.coverage.exec = ''
  #   (
  #     # change working directory to git toplevel
  #     cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)"
  #     docker run --rm -itv "$(pwd):/volume" xd009642/tarpaulin sh -c 'cargo tarpaulin "$@"'
  #   )
  # '';
}
