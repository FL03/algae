job("Build and Publish Crates") {
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo build --release
                cargo publish --all-features -p algae-core  --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN
                cargo publish --all-features -p algae-derive  --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN
                cargo publish --all-features -p algae-macros  --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN
                cargo publish --all-features -p algae-merkle  --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN
                cargo publish --all-features -p algae  --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN
            """
        }
    }
}