use std::env;

pub fn port() -> String {
    let mut p = String::from("3000");
    match env::var("PORT") {
        Ok(val) => p = val,
        Err(_e) => println!("using default port {}",  p),
    }
    return p;
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

}
