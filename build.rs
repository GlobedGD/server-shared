fn generate_for(name: &str) {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file(format!("./schema/{name}.capnp"))
        .output_path("schema/generated")
        .run()
        .unwrap_or_else(|e| panic!("capnpc failed for {name}: {e}"));
}

fn main() {
    if std::env::var("SERVER_SHARED_PREBUILT_DATA").is_ok() {
        return;
    }

    generate_for("shared");

    if std::env::var("CARGO_FEATURE_MAIN").is_ok() {
        generate_for("main");
    }

    if std::env::var("CARGO_FEATURE_GAME").is_ok() {
        generate_for("game");
    }

    if std::env::var("CARGO_FEATURE_SRVC").is_ok() {
        generate_for("srvc");
    }
}
