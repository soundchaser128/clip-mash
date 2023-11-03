project := justfile_directory()
frontend := project + "/frontend"
backend := project + "/backend"

@backend *cmd:
    cd {{backend}} && just {{cmd}}

@frontend *cmd:
    cd {{frontend}} && just {{cmd}}

format:
    just backend format
    just frontend format

check:
    just backend check
    just frontend check

build:
    just frontend build
    just backend build

setup:
    just frontend setup
    just backend setup
