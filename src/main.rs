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
          .ok()
          .expect("invalid path"),
       _ =>
         "{{.}}".to_string()
   };
   let input = if io::stdio::stdin_raw().isatty() {
     "{}".to_string()
   } else {
     io::stdin()
       .read_line()
       .ok()
       .expect("json expected")
   };
   let json = Json::from_str(&input)
     .ok()
     .expect("malformed json");
   let mut handlebars = Handlebars::new();
   handlebars.register_template_string("t", template)
     .ok()
     .expect("expected template");
   println!("{}", handlebars.render("t", &json)
                  .ok().unwrap());
}
