{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
  ];

  shellHook = ''
    export RUSTUP_HOME="$PWD/.rustup"
    export CARGO_HOME="$PWD/.cargo"
    export PATH="$CARGO_HOME/bin:$PATH"
    rustup default stable
  '';
}