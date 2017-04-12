extern crate liquid;

use self::liquid::{Context, Error, FilterError, Renderable, Template, Value};
use std::collections::HashMap;
use std::path::Path;

pub enum TemplateError {
    Unknown,
    Required,
    Parsing(Error),
}


pub fn template_file<T: Iterator<Item = (String, String)>>(file: &Path,
                                                           map: T)
                                                           -> Result<String, TemplateError> {
    match liquid::parse_file(file, Default::default()) {
        Ok(tmpl) => {
            let mut ctx = Context::new();
            for (k, v) in map {
                ctx.set_val(&k, Value::Str(v));
            }
            template(&tmpl, ctx)
        }
        Err(e) => Err(TemplateError::Parsing(e)),
    }
}

pub fn template_str(string: &str,
                    env: &HashMap<String, String>,
                    params: &HashMap<String, String>)
                    -> Result<String, TemplateError> {
    match liquid::parse(string, Default::default()) {
        Ok(tmpl) => {
            let mut ctx = Context::new();
            for (k, v) in env {
                ctx.set_val(k, Value::Str(v.clone()));
            }
            for (k, v) in params {
                ctx.set_val(k, Value::Str(v.clone()));
            }
            template(&tmpl, ctx)
        }
        Err(e) => {
            out!("ERROR parsing '{}' as template: {}", string, e);
            Ok(string.to_owned())
        }
    }
}

pub fn template(tmpl: &Template, mut ctx: Context) -> Result<String, TemplateError> {
    ctx.add_filter("required",
                   Box::new(|input, _args| {
        if let Value::Str(ref s) = *input {
            if s.is_empty() {
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
