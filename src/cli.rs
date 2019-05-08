use std::env;

#[derive(Debug)]
pub struct Parameters {
    pub username: String,
    pub password: String
}

impl Parameters {

    pub fn read() -> Parameters {
        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            println!("Usage: rabbitmq_init rabbitmq_username rabbitmq_password");
        }
        assert_eq!(3, args.len());
        Parameters {
            username: args[1].clone().to_string(),
            password: args[2].clone().to_string()
        }
    }

}