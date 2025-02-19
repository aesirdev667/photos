{ pkgs, ... }:
{
  env.TAURI_FRONTEND_PATH = "src-ui/";

  languages.javascript.enable = true;
  languages.javascript.bun.enable = true;
  languages.javascript.bun.package = pkgs.unstable.bun;
  languages.rust.enable = true;

  packages = with pkgs; [ cargo-tauri ];

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
      # change working directory to git toplevel
      cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)/src-ui"
      # execute command from here
      ${pkgs.lib.getExe pkgs.unstable.bun} x nuxi "$@"
    )
  '';

  scripts.cargo.exec = ''
    (
      # change working directory to git toplevel
      cd "$(${pkgs.lib.getExe pkgs.git} rev-parse --show-toplevel)/src-tauri"
      # execute command from here
      ${pkgs.lib.getExe pkgs.cargo} "$@"
    )
  '';
}
