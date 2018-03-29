use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use yaml_rust::yaml;

pub fn port() -> String {
    env::var("PORT").unwrap_or("3000".to_string())
}

pub fn language(lang: String) -> Result<yaml::Yaml, Error> {
    let mut f = File::open(["config/",lang.as_str(),".yml"].join("")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    return Ok(docs[0].clone());
}

#[cfg(test)]
mod tests {
    use config;
    use std::env;
    
    #[test]
    fn default_to_port_3k() {
        assert_eq!("3000", config::port());
    }

    #[test]
    fn uses_port_from_env() {
        env::set_var("PORT", "3030");
        assert_eq!("3030", config::port());
    }

    #[test]
    fn read_language_config_file() {
        let versions = config::language("test".to_string()).unwrap();
        assert_eq!("2.0.0", versions["latest"][0].as_str().unwrap());
        assert_eq!("1.2.3", versions["supported"][0].as_str().unwrap());
        assert_eq!("1.1.2", versions["supported"][1].as_str().unwrap());
        assert_eq!("1.0.3", versions["maintenance"][0].as_str().unwrap());
    }

}
