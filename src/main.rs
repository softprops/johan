extern crate "rustc-serialize" as serialize;
extern crate handlebars;

use handlebars::Handlebars;
use serialize::json::Json;
use std::old_io as io;
use std::os;

fn main() {
   let template = match os::args().as_slice() {
      [_, ref bars] =>
        io::File::open(&Path::new(&bars))
          .and_then(|mut f| f.read_to_string())
          .ok(),
       _ =>
         Some("{{.}}".to_string())
   };

   let json = if io::stdio::stdin_raw().isatty() {
     Some("{}".to_string())
   } else {
     io::stdin()
       .read_line()
       .ok()
   }.and_then(|input| Json::from_str(&input).ok());

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
