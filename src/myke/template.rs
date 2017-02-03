extern crate liquid;

use std::collections::HashMap;
use std::path::Path;
use self::liquid::{Renderable, Context, Value, FilterError, Template};

pub fn template_file<T: Iterator<Item=(String, String)>>(file: &Path, map: T)
                                                        -> Result<Option<String>, liquid::Error> {
    let tmpl = liquid::parse_file(file, Default::default()).unwrap();
    template(&tmpl, map)
}

pub fn template_str<T: Iterator<Item=(String, String)>>(string: &str, map: T)
                                                         -> Result<Option<String>, liquid::Error> {
    let tmpl = liquid::parse(string, Default::default()).unwrap();
    template(&tmpl, map)
}

pub fn template<T: Iterator<Item=(String, String)>>(tmpl: &Template, map: T)
                                                            -> Result<Option<String>, liquid::Error> {
    let mut ctx = Context::new();

    ctx.add_filter("required", Box::new(|input, _args| {
        if let &Value::Str(ref s) = input {
            if s.len() == 0 {
                Err(FilterError::InvalidType("Expected value".to_owned()))
            } else {
                Ok(Value::Str(s.clone()))
            }
        } else {
            Err(FilterError::InvalidType("Expected a string".to_owned()))
        }
    }));

    for (k, v) in map {
        ctx.set_val(k.as_str(), Value::Str(v));
    }
    tmpl.render(&mut ctx)
}
