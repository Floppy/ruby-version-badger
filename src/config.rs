use std::env;

pub fn port() -> String {
    env::var("PORT").unwrap_or("3000".to_string())
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
