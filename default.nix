# Define mozilla overlay
let
  rust_moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  glibc_overlay = (self: super: { gcc = self.gcc7; });
in
with import <nixpkgs> { overlays = [rust_moz_overlay];};
let
    beaglebone = import <nixpkgs> {
      system = "armv7l-linux";
    };

    cross = import <nixos-1903> {
       crossSystem = {
         config = "armv7l-unknown-linux-gnueabihf";
      };
    };

    rust_stable = latest.rustChannels.stable.rust.override {
        extensions = [
            "rust-src"
            # "rls-preview"
            # "rust-analysis"
            "rustfmt-preview"
        ];
        /* targets = [
            "armv7-unknown-linux-gnueabihf"
        ]; */
        # targetExtensions = [
        #     "rust-src"
        #     "rustfmt-preview"
        # ];
    };
    vscode_extensions = (with pkgs.vscode-extensions; [
      bbenoist.nix
      vscode-extensions.matklad.rust-analyzer
      tamasfe.even-better-toml
      yzhang.markdown-all-in-one
    ]) ++ pkgs.vscode-utils.extensionsFromVscodeMarketplace [
      {
        name = "better-comments";
        publisher = "aaron-bond";
        version = "2.1.0";
        sha256 = "0kmmk6bpsdrvbb7dqf0d3annpg41n9g6ljzc1dh0akjzpbchdcwp";
      }
      {
        name = "vscode-lldb";
        publisher = "vadimcn";
        version = "1.6.10";
        sha256 = "1q3d99l57spkln4cgwx28300d9711kc77mkyp4y968g3zyrmar88";
      }
      {
        name = "crates";
        publisher = "serayuzgur";
        version = "0.5.10";
        sha256 = "1dbhd6xbawbnf9p090lpmn8i5gg1f7y8xk2whc9zhg4432kdv3vd";
      }
    ];
  vscode-with-extensions = pkgs.vscode-with-extensions.override {
      vscodeExtensions = vscode_extensions;
    };
in
mkShell {
  name = "rust-env";
  nativeBuildInputs = with buildPackages; [

    rust_stable
    cargo-xbuild
    vscode-with-extensions
    #cross.binutils
    # cross.stdenv.glibc

    /* vscode # visual studop code */

    # latest.rustChannels.nightly.rust.override {extensions = [ "rust-src" ];}
    # rustup
    #pkg-config
    # git

    # rust_latest_override
    # unstable.rust-analyzer
    /* protobuf
    protobuf3_9
    sbt */
    # protobuf3_9


    # rls
    # lldb
  ];

  buildInputs = [
    cross.stdenv.cc
    cross.binutils
  ];

  # This environment variable specifies linker for armv7-unknown-linux-gnueabihf target
  # CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = "armv7l-unknown-linux-gnueabihf-gcc";
  # CARGO_TARGET_DIR = "/home/misha/NFS/public/cmpt433";

  # RUSTFLAGS="-C link-arg=-Wl,-dynamic-linker,/lib/ld-linux-armhf.so.3";
  #RUSTFLAGS = "";

}
