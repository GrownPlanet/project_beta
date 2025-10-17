build:
    wasm-pack build --target web
    rm -rf ./website/pkg
    mv ./pkg ./website/pkg

run:
    just build
    cd website && python -m http.server
