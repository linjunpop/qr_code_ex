#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;

extern crate qrcode;

use qrcode::render::svg;
use qrcode::QrCode;

use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
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

fn generate_svg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let data: &str = try!(args[0].decode());

    let code = QrCode::new(data.as_bytes()).unwrap();

    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    Ok((atoms::ok(), image).encode(env))
}

fn generate_string<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let data: &str = try!(args[0].decode());

    let code = QrCode::new(data.as_bytes()).unwrap();
    let string = code.render::<char>().build();

    Ok((atoms::ok(), string).encode(env))
}
