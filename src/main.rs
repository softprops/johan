extern crate atty;
extern crate rustc_serialize;
extern crate clap;
extern crate handlebars;

use clap::App;
use handlebars::Handlebars;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::exit;

fn main() {

  fn ok<S: Into<String>>(txt: S) {
    write!(&mut std::io::stdout(), "{}", txt.into());
    exit(0)
  }

  fn err<S: Into<String>>(txt: S) {
    write!(&mut std::io::stderr(), "{}", txt.into());
    exit(1)
  }

  let matches = App::new("johan")
    .about("applies json data to handlebars templates")
    .args_from_usage(
      "-t, --template=[TEMPLATE] 'handlebars template path. defaults to handlebars src {{.}}'
       [INPUT] 'json data. will read from std input'"
     )
    .get_matches();
  let bars  = matches.value_of("TEMPLATE");
  let input = matches.value_of("INPUT").map(|i| i.to_owned()).or_else(|| {
    if atty::is() {
      let mut raw = String::new();
      io::stdin()
        .read_line(&mut raw)
        .map(|_| raw)
        .ok()
    } else {
      None
    }
  });

  let template: Option<String> =
    match bars {
      Some(bars) =>
        File::open(bars)
      .and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents).map(|_| contents)
      })
      .ok(),
    _ =>
      Some("{{.}}".to_string())
   };

  let json = input.and_then(|input| Json::from_str(&input).ok());

  let mut handlebars = Handlebars::new();
  let rendered = template
    .and_then(|bars| handlebars.register_template_string("t", bars).ok())
    .and_then(|_| json)
    .and_then(|json| handlebars.render("t", &json).ok());

    match rendered {
      Some(r) => ok(r),
      _ => err("failed to render")
    }
}
