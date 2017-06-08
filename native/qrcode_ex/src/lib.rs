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
    [("add", 2, add)],
    None
}

fn add<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

// fn generate_svg<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
// fn generate_svg() {
//     let code = QrCode::with_version(b"01234567", Version::Micro(2), EcLevel::L).unwrap();
//     let image = code.render()
//         .min_dimensions(200, 200)
//         .dark_color(svg::Color("#800000"))
//         .light_color(svg::Color("#ffff80"))
//         .build();

//     OK((atoms::ok(), ).encode(env))
// }
