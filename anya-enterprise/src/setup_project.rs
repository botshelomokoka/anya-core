impl ProjectSetup {
    pub fn new(user_type: UserType, user_data: HashMap<String, String>) -> Result<Self, Box<dyn Error>> {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = slog::Logger::root(drain, slog::o!());
        
        Ok(Self {
            // ... rest of the struct initialization ...
        })
    }
}
