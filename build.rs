fn main() {
    // cargo standard directive: re-run this build script when the proto
    // files change, so generated code is not stale (cached OUT_DIR).
    println!("cargo:rerun-if-changed=protos/rendezvous.proto");
    println!("cargo:rerun-if-changed=protos/message.proto");

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
