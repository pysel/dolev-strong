extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("../src/prototypes")
        .inputs(&["dolevstrong/messages.proto", "dolevstrong/basic.proto"])
        .include("dolevstrong")
        .run()
        .expect("Codegen failed.");
}