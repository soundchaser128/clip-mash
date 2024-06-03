set shell := ["nu", "-c"]

project := justfile_directory()
frontend := project + "/frontend"
backend := project + "/backend"
e2e := project + "/e2e-tests"
client := project + "/rust-client"

default:
  @just --list

@backend *cmd:
    cd {{backend}}; just {{cmd}}

@frontend *cmd:
    cd {{frontend}}; just {{cmd}}

@e2e *cmd:
    cd {{e2e}}; just {{cmd}}

@client *cmd:
    cd {{client}}; just {{cmd}}

format:
    just backend format
    just frontend format
    just client format

check:
    just backend check
    just frontend check
    just client check

build:
    just frontend build
    just backend build
    just client build

setup:
    just frontend setup
    just backend setup

fix:
    just backend fix
    just frontend fix
    just client fix

generate-code:
    just backend generate-openapi-spec
    just frontend generate-client
    just backend generate-rust-client
    rm api-docs.json
