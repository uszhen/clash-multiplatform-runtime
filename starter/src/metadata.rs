use std::{collections::HashMap, error::Error, io::Read, path::Path};

pub struct Metadata {
    pub is_premium: bool,
}

impl Metadata {
    fn new_from_hash_map(map: &HashMap<&str, &str>) -> Result<Metadata, Box<dyn Error>> {
        let is_premium = if let Some(text) = map.get("Clash-Premium") {
            *text == "true"
        } else {
            return Err("property 'Clash-Premium' not found".into());
        };

        Ok(Metadata { is_premium })
    }
}

pub fn resolve_app_metadata(classpath: &Path) -> Result<Metadata, Box<dyn Error>> {
    let file = std::fs::File::open(classpath)?;

    let mut zip = zip::ZipArchive::new(file)?;
    let mut entry = zip.by_name("META-INF/MANIFEST.MF")?;

    let mut manifest = String::with_capacity(entry.size() as usize);
    entry.read_to_string(&mut manifest)?;

    let maps = manifest
        .lines()
        .map(|line| line.splitn(2, ":"))
        .map(|mut segments| (segments.next(), segments.next()))
        .filter(|pair| pair.0.is_some() && pair.1.is_some())
        .map(|pair| (pair.0.unwrap().trim(), pair.1.unwrap().trim()))
        .collect::<HashMap<&str, &str>>();

    Metadata::new_from_hash_map(&maps)
}
