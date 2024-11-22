use tonic_build::configure;

fn main() {


    println!("starting");
    configure()
        .out_dir(".")
        .compile(
            &[
                "proto/shredstream.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
