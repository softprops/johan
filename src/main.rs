#![feature(old_io, plugin)]
#![plugin(docopt_macros)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate handlebars;

use handlebars::Handlebars;
use rustc_serialize::json::Json;
use std::fs::File;
use std::path::Path;
use std::io;
use std::old_io;
use std::io::Read;
use std::env;

//docopt!(Args derive Debug, "
//Usage: johan [options] [-]
//       johan (--help | --version)

//Options:
//  -h, --help          show this help content
//  -f, --file FILE     path to handlebars (.hbs) template
//  -t, --template STR  a raw handlebars template string
//  -j, --json STR      json data
//", flag_file: Option<Path>, flag_template: Option<String>, flag_json: Option<//String>);

fn main() {

  //let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
  //println!("args {:?}", args);
  
  let template: Option<String> = match env::args_os().as_slice() {
      [_, ref bars] =>
        File::open(bars)
          .and_then(|mut f| {
              let mut contents = String::new();
              f.read_to_string(&mut contents).map(|_| contents)
          })
          .ok(),
       _ =>
         Some("{{.}}".to_string())
   };

   let opt: Option<String> = if old_io::stdio::stdin_raw().isatty() {
     Some("{}".to_string())
   } else {
     let mut raw = String::new();
     io::stdin()
       .read_line(&mut raw)
       .map(|_| raw)
       .ok()
   };
   let json = opt.and_then(|input| Json::from_str(&input).ok());

   let mut handlebars = Handlebars::new();
   let rendered = template
      .and_then(|bars| handlebars.register_template_string("t", bars).ok())
      .and_then(|_| json)
      .and_then(|json| handlebars.render("t", &json).ok());

    match rendered {
      Some(r) => println!("{}", r),
      _ => println!("failed to render")
    }
}
