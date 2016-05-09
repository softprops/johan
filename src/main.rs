extern crate atty;
extern crate clap;
extern crate handlebars;
extern crate serde_json;

use clap::App;
use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::exit;

fn handlebars() -> Handlebars {
    let mut hbs = Handlebars::new();
    fn json_helper(c: &Context,
                   h: &Helper,
                   _: &Handlebars,
                   rc: &mut RenderContext)
                   -> Result<(), RenderError> {
        let param = h.params().get(0).unwrap();
        let val = c.navigate(rc.get_path(), param);
        let rendered = serde_json::to_string(&val).unwrap();
        try!(rc.writer.write(rendered.into_bytes().as_ref()));
        Ok(())
    }
    hbs.register_helper("json", Box::new(json_helper));
    hbs
}

fn template(match_template: Option<&str>, match_file: Option<&str>) -> Option<String> {
    match_template.map(|t| t.to_owned()).or(match match_file {
        Some(bars) => {
            File::open(bars)
                .and_then(|mut f| {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).map(|_| contents)
                })
                .ok()
        }
        _ => Some("{{json .}}".to_string()),
    })
}

fn json(input: Option<&str>, inject_env: bool) -> Option<serde_json::Value> {
    match input {
        Some("-") => None,
        None => Some("{}"),
        value => value
    }
    .map(|i| i.to_owned())
               .or_else(|| {
                   if atty::is() {
                       let mut raw = String::new();
                       io::stdin()
                           .read_line(&mut raw)
                           .map(|_| raw)
                           .ok()
                   } else {
                       None
                   }
               })
        .and_then(|input| serde_json::from_str(&input).ok())
        .and_then(|json| {
            let mut wrapper = BTreeMap::new();
            wrapper.insert("data", json);
            if inject_env {
                let mut env_map = BTreeMap::new();
                for (k, v) in env::vars() {
                    env_map.insert(k,v);
                }
                let env = serde_json::value::to_value(&env_map);

                wrapper.insert("env", env);
            }
            Some(serde_json::value::to_value(&wrapper))
        })
}

fn main() {
    let matches = App::new("johan")
                      .about("applies json data to handlebars templates")
                      .args_from_usage("-t, --template=[TEMPLATE] 'inline handlebars template string. defaults to printing raw json unless file is provided'
                                        -f, --file=[TEMPLATE_FILE] 'handlebars template file path'
                                        -e, --env... 'inject env'
                                        [INPUT] 'json data. will read from std input when - is provided as an argument. defaults to {} otherwise'")
                      .get_matches();
    let rendered = json(matches.value_of("INPUT"), matches.occurrences_of("env") > 0).and_then(|json| {
        template(matches.value_of("template"), matches.value_of("file"))
            .and_then(|bars| handlebars().template_render(&bars, &json).ok())
    });
    match rendered {
        Some(r) => println!("{}", r),
        _ => {
            let _ = write!(&mut std::io::stderr(), "{}", "failed to render");
            exit(1)
        }
    }
}
