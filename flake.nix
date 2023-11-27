{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: {
    defaultPackage.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nginx.override {
      modules = [ nixpkgs.legacyPackages.x86_64-linux.nginxModules.echo ];
    };
  };
}
