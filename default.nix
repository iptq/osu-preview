{ napalm }:

napalm.buildPackage ./. {
  name = "osu-preview";
  npmCommands = [ "npm install" "npm run build:all" ];
}
