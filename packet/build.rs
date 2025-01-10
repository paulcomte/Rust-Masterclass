extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "packets/ping.proto",
            "packets/authenticate.proto",
            "packets/packet.proto",
        ],
        &["packets/"],
    )
    .unwrap();
}
