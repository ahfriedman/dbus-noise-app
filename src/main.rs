use dbus::blocking::Connection;
use std::time::Duration;
use dbus_crossroads::{Crossroads, Context};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let c = Connection::new_session()?;

    c.request_name("com.ahfriedman.white_noise", false, true, false)?; 

    let mut cr = Crossroads::new(); 

    let token = cr.register("com.ahfriedman.white_noise", |b|{

        b.method("toggle", ("name", ), ("reply",), |_, _, (name, ): (String, )|{
            println!("hello!");
            Ok(("Idk", ))
        });
    });

    cr.insert("/toggle", &[token], ());
    cr.serve(&c);

    Ok(())
}
