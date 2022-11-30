use anyhow::Result;
use crux_core::{
    http::{HttpRequest, HttpResponse},
    key_value::{KeyValueRequest, KeyValueResponse},
    platform::PlatformResponse,
    time::TimeResponse,
    typegen::TypeGen,
    Request,
};
use shared::{app::platform::PlatformEvent, Effect, Event, ViewModel};
use std::path::PathBuf;

fn main() {
    let mut gen = TypeGen::new();

    register_types(&mut gen).expect("type registration failed");

    let output_root = PathBuf::from("./generated");

    gen.swift("shared_types", output_root.join("swift"))
        .expect("swift type gen failed");

    gen.java(
        "com.redbadger.catfacts.shared_types",
        output_root.join("java"),
    )
    .expect("java type gen failed");

    gen.typescript("shared_types", output_root.join("typescript"))
        .expect("typescript type gen failed");
}

fn register_types(gen: &mut TypeGen) -> Result<()> {
    gen.register_type::<Request<Effect>>()?;

    gen.register_type::<Effect>()?;
    gen.register_type::<HttpRequest>()?;
    gen.register_type::<KeyValueRequest>()?;

    gen.register_type::<Event>()?;
    gen.register_type::<HttpResponse>()?;
    gen.register_type::<KeyValueResponse>()?;
    gen.register_type::<TimeResponse>()?;

    gen.register_type::<PlatformEvent>()?;
    gen.register_type::<PlatformResponse>()?;

    gen.register_type::<ViewModel>()?;
    Ok(())
}
