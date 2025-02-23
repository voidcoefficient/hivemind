{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/

  # https://devenv.sh/packages/
  packages = with pkgs; [ nats-server nats-streaming-server natscli nats-top nsc ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;
  # languages.rust.channel = "nightly";

  # https://devenv.sh/processes/
  processes.nats-server.exec = "nats-server -js -c js.conf";

  # https://devenv.sh/services/

  # https://devenv.sh/scripts/

  # https://devenv.sh/tasks/

  # https://devenv.sh/tests/

  # https://devenv.sh/git-hooks/

  # See full reference at https://devenv.sh/reference/options/
}
