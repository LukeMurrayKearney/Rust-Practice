pub struct Agent {
    pub agent_type: u32,
    pub dead: Dead,
}

pub enum Dead{
    Alive,
    Dead(u32),
}

fn main() {
    let mut agent = Agent {
        agent_type: 3,
        dead: Dead::Dead(3),
    };

    match agent.dead {
        Dead::Dead(number) => println!("dead, at {}",number),
        _ => println!("alive"),
    }
    agent.dead = Dead::Alive;
    match agent.dead {
        Dead::Dead(number) => println!("dead, at {}",number),
        _ => println!("alive"),
    }
}
