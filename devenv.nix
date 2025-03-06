{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.DATABASE_URL = "postgres://postgres:postgres@localhost:5432/hivemind";

  # https://devenv.sh/packages/
  packages = with pkgs; [ sqlx-cli nats-server nats-streaming-server natscli nats-top nsc ];

  # https://devenv.sh/languages/
  languages.deno.enable = true;

  # https://devenv.sh/processes/
  processes.nats-server.exec = "nats-server -js -c js.conf";

  # https://devenv.sh/services/
  services.adminer.enable = true;
  services.postgres = {
    enable = true;
    createDatabase = true;
    listen_addresses = "127.0.0.1";
    initialDatabases = [{
      name = "hivemind";
      user = "postgres";
      pass = "postgres";
    }];
    extensions = extensions: with extensions; [ pg_uuidv7 ];
  };

  # https://devenv.sh/scripts/

  # https://devenv.sh/tasks/

  # https://devenv.sh/tests/

  # https://devenv.sh/git-hooks/

  # See full reference at https://devenv.sh/reference/options/
}
