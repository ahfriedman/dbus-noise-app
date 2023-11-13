use dbus::blocking::Connection;
// use std::time::Duration;
use dbus_crossroads::{Crossroads};
use std::process::{Command, Child};
use std::cell::RefCell; 
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let c = Connection::new_session()?;

    c.request_name("com.ahfriedman.white_noise", false, true, false)?; 

    let mut cr = Crossroads::new(); 

    let token = cr.register("com.ahfriedman.white_noise", |b|{

        let mut generator : Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None)); 
        let tg = generator.clone(); 
        b.method("toggle", (), ("reply",),move |_, _, _ : ( )|{
            let mut genRef = tg.lock().unwrap();
            match &mut *genRef {
                Some(child) => {child.kill().expect("!kill"); *genRef = None; }
                None => {
                           *genRef = Some(Command::new("play")
                                .arg("-q")
                                .arg("-n")
                                .arg("-t")
                                .arg("alsa")
                                .arg("synth")
                                .arg("brownnoise")
                                .arg("lowpass")
                                .arg("-1")
                                .arg("1k")
                                .arg("gain")
                                .arg("-10")
                                .spawn().expect("Couldn't run 'play'"));
                            }
            }
            println!("{}", !genRef.is_none());
            Ok((!genRef.is_none(), ))
        });

        b.method("status", (), ("reply", ), move |_, _, () : ()| {
            let mut genRef = generator.lock().unwrap();
            Ok((!genRef.is_none(), ))
        });
    });

    cr.insert("/toggle", &[token], ());
    cr.serve(&c);

    Ok(())
}
