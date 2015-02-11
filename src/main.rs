extern crate "rustc-serialize" as serialize;
extern crate handlebars;

use std::old_io as io;
use serialize::json::Json;
use handlebars::Handlebars;

fn main() {
   let input = if io::stdio::stdin_raw().isatty() {
     io::stdin().read_line().ok().expect("json expected")
   } else {
     "{}".to_string()
   };
   let json = Json::from_str(&input).ok().expect("malformed json");
   let mut handlebars = Handlebars::new();
   handlebars.register_template_string("t","hello {{foo}}".to_string()).ok().expect("expected template");
   println!("{}", handlebars.render("t", &json).ok().unwrap());
}
