extern crate amqp;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;
mod rabbitmq;
mod cli;

use settings::Settings;
use rabbitmq::RabbitMQ;
use cli::Parameters;

fn main() {
    println!("Initializing rabbitMq for piTrader us");
    let parameters = Parameters::read();
    println!("Parameters; {:?}", parameters);
    let settings = Settings::new().unwrap();
    println!("Settings: {:?}", settings);
    let mut mq = RabbitMQ::new(
        &settings.rabbit_mq.address, 
        &parameters.username,
        &parameters.password,
        settings.rabbit_mq.pre_fetch_items
    );
    println!("Connected to rabbitmq on channel {}", mq.channel.id);
    mq.create_exchange(&settings.exchange.ticket_reader_data);
    println!("Created 'ticket_reader_data' exchange");
    mq.close();
    println!("Connection to rabbitMq closed");
}
