build:
    wasm-pack build --target web

run:
    just build
    python -m http.server

commit:
    just build
    rm pkg/.gitignore
    git add -A
    git commit
