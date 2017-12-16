extern crate sprig;
extern crate gtmpl_value;

use gtmpl::{Context, Template, Func};
use self::sprig::SPRIG;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;

pub fn template_file<T: Iterator<Item = (String, String)>>(file: &Path,
                                                           iter: T)
                                                           -> Result<String, String> {
    let mut f = File::open(file).map_err(|e| format!("file not found: {}", e))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents).map_err(|e| format!("unable to read file: {}", e))?;
    let map: HashMap<String, String> = iter.collect();
    let tmpl = create_template(&contents)?;
    tmpl.render(&Context::from(map)?)
}

pub fn template_str(string: &str,
                    env: &HashMap<String, String>,
                    params: &HashMap<String, String>)
                    -> Result<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in env {
        map.insert(k.clone(), v.clone());
    }
    for (k, v) in params {
        map.insert(k.clone(), v.clone());
    }
    let tmpl = create_template(string)?;
    tmpl.render(&Context::from(map)?)
}

fn create_template(string: &str) -> Result<Template, String> {
    let mut tmpl = Template::with_name("");
    tmpl.add_funcs(SPRIG as &[(&str, Func)]);
    tmpl.add_func("required", required);
    tmpl.parse(string)?;
    Ok(tmpl)
}

gtmpl_fn!(
    fn required(arg: String) -> Result<String, String> {
        if arg.is_empty() {
            Err(String::from("missing required argument"))
        } else {
            Ok(arg)
        }
    }
);
