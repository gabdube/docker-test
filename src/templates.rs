use crate::error::InnerServerError;
use std::{fs, path};


#[repr(u32)]
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum TemplateId {
    Index,
    MaxId,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Templates {
    inner: Vec<String>
}

impl Templates {

    pub fn get(&self, id: TemplateId) -> String {
        let index = id as usize;
        assert!(index < self.inner.len(), "Template ID {:?} out of bounds", id);
        self.inner[index].to_string()
    }

}

fn read_template(name: &'static str) -> Result<String, InnerServerError> {
    let name = format!("./assets/templates/{}", name);
    let path = path::Path::new(&name);

    match fs::read_to_string(path) {
        Ok(src) => Ok(src),
        Err(err) => Err(InnerServerError::failed_to_load_template(format!("Failed to load template at {:?}: {}", name, err)))
    }
}

pub fn load() -> Result<Templates, InnerServerError> {
    let mut inner = Vec::with_capacity(TemplateId::MaxId as usize);

    inner.push(read_template("index.html")?);

    assert!(inner.len() == (TemplateId::MaxId as usize), "MaxId != templates count. Missing template implementation");

    let t = Templates {
        inner
    };

    Ok(t)
}
