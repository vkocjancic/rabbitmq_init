
use amqp::{
    Basic,
    Channel,
    Options,
    Session,
    Table,
};

pub struct RabbitMQ {
    session: Session,
    pub channel: Channel,
}

impl RabbitMQ {
    pub fn new(address: &str, login: &str, password: &str, prefetch_count: u16) -> RabbitMQ {
        let mut amqp_session = Session::new(
            Options {
                host: address.to_string(), 
                login: login.to_string(), 
                password: password.to_string(), 
                .. Default::default()} ).ok().expect("Can't create session");
        let mut amqp_channel = amqp_session.open_channel(1).unwrap();
        if prefetch_count != 0 {
            amqp_channel.basic_prefetch(prefetch_count).unwrap();
        }
        RabbitMQ {
            session: amqp_session,
            channel: amqp_channel
        }
    }

    pub fn create_exchange(&mut self, exchange_name: &str) {
        self.channel.exchange_declare(
            exchange_name,
            "fanout",
            false,
            true,
            false,
            false,
            false,
            Table::new(),
        ).unwrap();
    }

    pub fn close(&mut self) {
        const CLOSE_REPLY_CODE: u16 = 200;
        const CLOSE_REPLY_TEXT: &str = "closing producer";
        self.channel.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        ).unwrap();
        self.session.close(
            CLOSE_REPLY_CODE,
            CLOSE_REPLY_TEXT,
        );
    }
}