[private]
@list:
    just -l

# fetch syn.json & build ssttt
build-latest:
    curl https://raw.githubusercontent.com/dtolnay/syn/master/syn.json -o syn.json
    cargo run -p codegen

# build ssttt without download syn.json
build:
    cargo run -p codegen
