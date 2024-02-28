{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };
  outputs = inputs: with inputs; 
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in
  {
    packages.x86_64-linux = rec {
      default = nuxhxboard;
      nuxhxboard = pkgs.callPackage ./package.nix {};
    };
    devShell.x86_64-linux = rec {
        default = nuxhxboard
        nuxhxboard = pkgs.callPackage ./shell.nix {};
    };
  };
}