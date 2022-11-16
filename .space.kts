job("(algae) Rust: Build and test workspace") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        env["CARGO_REGISTRY_TOKEN"] = Secrets("cargo_registry_token")
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all-features
            """
        }
    }
}

job("(algae) Rust: Publish crates") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
    }
    container(displayName = "Rust", image = "rust") {
        env["CARGO_REGISTRY_TOKEN"] = Secrets("cargo_registry_token")

        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo build --release
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae-graph
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae-merkle
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae-mmr
                cargo publish --all-features --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN -p algae
            """
        }
    }
}