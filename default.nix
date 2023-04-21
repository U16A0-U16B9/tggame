with builtins;
{ pkgs ? import
  (fetchTarball {
    name = "nixpkgs-unstable-2023-04-16";
    url = "https://github.com/NixOS/nixpkgs/archive/29176972b4be60f7d3eb3101f696c99f2e6ada57.tar.gz";
    sha256 = "1c8rjv87fv9bfnp6j4g5wnsf8lyl9aldvy947ihshkd178fpfw23";
  })
  {}
  , overlays ? [ ]
  , config ? { }
  , system ? builtins.currentSystem
}:
let
  name = "tggame";
  _ = with pkgs; rec {
    pg = postgresql_14;
  };
  env_bootstrap = with pkgs; writeScriptBin "__env_bootstrap" ''cp .env.default .env'';

  extensions = [ "pgcrypto" "uuid-ossp" ];
  pg_ctl = ''${_.pg}/bin/pg_ctl -o "-k '$PGDATA'" -D "$PGDATA"'';
  _psql = x: ''${_.pg}/bin/psql -d ${x} -h localhost -p "$PGPORT" -c'';
  psql = _psql "postgres";
  psql_db = _psql "$DATABASE_NAME";
  create_db = ''CREATE DATABASE \"$DATABASE_NAME\";'';
  create_ext = builtins.concatStringsSep "\n" (map (x: ''CREATE EXTENSION IF NOT EXISTS \"${x}\";'') extensions);
  create_user = ''
    CREATE USER \"$DATABASE_USERNAME\" WITH ENCRYPTED PASSWORD '$DATABASE_PASSWORD';
    GRANT ALL PRIVILEGES ON DATABASE \"$DATABASE_NAME\" TO \"$DATABASE_USERNAME\";
    ALTER USER \"$DATABASE_USERNAME\" CREATEDB;
  '';
  grant_schema = ''
    GRANT ALL ON SCHEMA public TO \"${name}\";
  '';

  pg_bootstrap = with pkgs; writeScriptBin "__pg_bootstrap" ''
    ${_.pg}/bin/initdb -E UTF8 "$PGDATA"
    ${pg_ctl} start
    ${psql} "${create_db}"
    ${psql} "${create_ext}"
    ${psql} "${create_user}"
    ${psql_db} "${create_ext}"
    ${psql_db} "${grant_schema}"
    ${pg_ctl} stop
  '';
  pg_run = with pkgs; writeScriptBin "__pg_run" ''${_.pg}/bin/postgres -k "$PGDATA" -D "$PGDATA" -p "$PGPORT"'';


  tools = with pkgs; {
    cli = [
      coreutils
      diesel-cli
      overmind
    ];
    scipts = [
      env_bootstrap
      pg_bootstrap
      pg_run
    ];
  };
  paths = pkgs.lib.flatten [ (builtins.attrValues tools) ];
in
pkgs.buildEnv {
  inherit name paths;
  buildInputs = paths;
}

