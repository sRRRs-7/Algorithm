use std::path::PathBuf;
use clap::{ App, Arg};
use qrcode::QrCode;
use image::Luma;


pub fn main() {
    qr_code()
}

struct GenerateOptions {
    text: String,
    output: Option<PathBuf>
}

pub fn qr_code() {
    let app = App::new("qr-text")
        .version("0.12.0")
        .author("srrrs")
        .about("Generate QRcode of specified String")
        .arg(
            Arg::with_name("OUTPUT")
                .short('o')
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .help("Indicate output filename"),
        ).arg(
            Arg::with_name("TEXT")
                .required(true)
                .help("Indicate embed QRcode of String"),
        ).get_matches();

        let text = String::from(app.value_of("TEXT").unwrap());
        let output = app.value_of("OUTPUT").map(PathBuf::from);

        let generate = GenerateOptions{text, output};

        let qr = QrCode::new(generate.text.as_bytes()).unwrap();
        let image = qr.render::<Luma<u8>>().build();
        image.save("./QRcode.png").unwrap();

        println!("{:?}", generate.output);

}