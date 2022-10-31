job("Build and Publish Crates") {
    container(displayName = "Rust", image = "rust") {
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo build --release
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae-merkle
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae-mmr
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae

            """
        }
    }
}