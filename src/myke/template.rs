extern crate liquid;

use std::collections::HashMap;
use std::path::Path;
use self::liquid::{Renderable, Context, Value, FilterError, Template};

pub enum TemplateError {
    Unknown,
    Required,
}


pub fn template_file<T: Iterator<Item=(String, String)>>(file: &Path, map: T)
                                                         -> Result<String, TemplateError> {
    let tmpl = liquid::parse_file(file, Default::default()).unwrap();
    let mut ctx = Context::new();
    for (k, v) in map {
        ctx.set_val(k.as_str(), Value::Str(v));
    }
    template(&tmpl, ctx)
}

pub fn template_str(string: &str, env: &HashMap<String, String>, params: &HashMap<String, String>)
                    -> Result<String, TemplateError> {
    let tmpl = liquid::parse(string, Default::default()).unwrap();
    let mut ctx = Context::new();
    for (k, v) in env {
        ctx.set_val(k.as_str(), Value::Str(v.clone()));
    }
    for (k, v) in params {
        ctx.set_val(k.as_str(), Value::Str(v.clone()));
    }
    template(&tmpl, ctx)
}

pub fn template(tmpl: &Template, mut ctx: Context)
                -> Result<String, TemplateError> {
    ctx.add_filter("required", Box::new(|input, _args| {
        if let &Value::Str(ref s) = input {
            if s.len() == 0 {
                return Err(FilterError::InvalidType("Expected value".to_owned()));
            }
        }
        Ok(input.clone())
    }));

    match tmpl.render(&mut ctx) {
        Ok(Some(s)) => Ok(s),
        Err(liquid::Error::Filter(_)) => Err(TemplateError::Required),
        _ => Err(TemplateError::Unknown),
    }
}
