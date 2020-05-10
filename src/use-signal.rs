use crossbeam_channel::{bounded, select, tick, Receiver};
use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread, time::Duration};

fn recieve_signals() -> Result<Receiver<i32>, Box<dyn Error>> {
    //他のシグナルを受ける場合、例えば以下のように記載する
    // Signals::new(&[SIGINT, SIGHUP])?;
    let signals = Signals::new(&[SIGINT])?;
    let (sender, receiver) = bounded(100);
    thread::spawn(move || {
        for sig in signals.forever() {
            let _ = sender.send(sig);
        }
    });
    Ok(receiver)
}

fn main() -> Result<(), Box<dyn Error>> {
    let receiver = recieve_signals()?;
    let ticks = tick(Duration::from_secs(10));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("processing");
            }
            recv(receiver) -> sig => {
                match sig{
                    Ok(sig) => {
                        println!();
                        match sig {
                            SIGINT => println!("recive: SIGINT"),
                            _ => println!("recive:{}",sig),
                        }
                        println!("done");
                    },
                    Err(_) => (),
                }
                break;
            }
        }
    }

    Ok(())
}
