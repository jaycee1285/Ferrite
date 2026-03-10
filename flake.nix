{
  description = "Ferrite - a fast, lightweight text editor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ]
      (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          lib = pkgs.lib;
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

          linuxBuildInputs = with pkgs; [
            gtk3
            fontconfig
            freetype
            libxkbcommon
            wayland
            wayland-scanner
            vulkan-loader
            libGL
            libx11
            xorg.libX11
            libxcursor
            xorg.libXcursor
            libxi
            xorg.libXi
            libxrandr
            xorg.libXrandr
            libxcb
            xorg.libxcb
            xorg.libXext
            xorg.libXrender
            xorg.libXfixes
            xorg.libXinerama
            xorg.libXdamage
            xorg.libXcomposite
            xorg.libXxf86vm
          ];

          darwinFrameworks = with pkgs.darwin.apple_sdk.frameworks; [
            AppKit
            Cocoa
            CoreFoundation
            CoreGraphics
            CoreServices
            Foundation
            Metal
            QuartzCore
          ];

          ferrite = pkgs.rustPlatform.buildRustPackage {
            pname = "ferrite";
            version = cargoToml.package.version;

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = [ pkgs.pkg-config ]
              ++ lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.wrapGAppsHook3 ];

            buildInputs = lib.optionals pkgs.stdenv.hostPlatform.isLinux linuxBuildInputs
              ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin darwinFrameworks;

            doCheck = false;

            postInstall = lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
              install -Dm644 assets/linux/io.github.olaproeis.Ferrite.desktop \
                $out/share/applications/ferrite.desktop
              for size in 16 32 48 64 128 256 512; do
                install -Dm644 assets/icons/linux/''${size}x''${size}/ferrite.png \
                  $out/share/icons/hicolor/''${size}x''${size}/apps/ferrite.png
              done
            '';

            meta = with lib; {
              description = "A fast, lightweight text editor for Markdown, JSON, and more";
              homepage = "https://github.com/OlaProeis/Ferrite";
              license = licenses.mit;
              mainProgram = "ferrite";
              platforms = platforms.linux ++ platforms.darwin;
            };
          };
        in
        {
          packages = {
            default = ferrite;
            ferrite = ferrite;
          };

          apps.default = {
            type = "app";
            program = "${ferrite}/bin/ferrite";
          };

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.rustc
              pkgs.cargo
              pkgs.rustfmt
              pkgs.clippy
              pkgs.rust-analyzer
              pkgs.pkg-config
            ]
            ++ lib.optionals pkgs.stdenv.hostPlatform.isLinux linuxBuildInputs
            ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin darwinFrameworks;

            LD_LIBRARY_PATH = lib.optionalString pkgs.stdenv.hostPlatform.isLinux
              (lib.makeLibraryPath linuxBuildInputs);

            PKG_CONFIG_PATH = lib.optionalString pkgs.stdenv.hostPlatform.isLinux
              "${pkgs.fontconfig.dev}/lib/pkgconfig:${pkgs.freetype.dev}/lib/pkgconfig";

            shellHook = ''
              echo "Ferrite Nix dev shell ready."
              echo "Run cargo commands normally, e.g. cargo run"
            '';
          };

          checks.default = ferrite;
        }
      );
}
