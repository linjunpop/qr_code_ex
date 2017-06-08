#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

extern crate qrcode;

use qrcode::{QrCode, Version, EcLevel};
use qrcode::render::svg;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.QRCode",
    [
        ("generate_svg", 1, generate_svg),
        ("generate_string", 1, generate_string)
    ],
    None
}

fn generate_svg<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let data: &str = try!(args[0].decode());

    let code = QrCode::new(data.as_bytes()).unwrap();

    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    Ok((atoms::ok(), image).encode(env))
}

fn generate_string<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let data: &str = try!(args[0].decode());

    let code = QrCode::new(data.as_bytes()).unwrap();
    let string = code.render::<char>()
        .build();

    Ok((atoms::ok(), string).encode(env))
}
