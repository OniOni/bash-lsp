{ pkgs ? import (fetchTarball "channel:nixos-23.05") {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.shellcheck
    pkgs.python3
  ];
}
