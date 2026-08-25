use crossbeam_channel::{Receiver, TryRecvError};
use protocol::{DesktopToEspPacket, encode_desktop_to_esp_packet};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    thread,
};

pub fn robot_worker(
    mut port: Option<Box<dyn serialport::SerialPort>>,
    receiver: Receiver<DesktopToEspPacket>,
    busy: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match receiver.try_recv() {
                Ok(p) => {
                    let byte = encode_desktop_to_esp_packet(p);
                    if let Some(port) = port.as_mut() {
                        port.write(&[byte]).unwrap();
                        port.flush().unwrap();
                    }
                    // :/
                    if p == DesktopToEspPacket::Shutdown {
                        break;
                    } else {
                        busy.store(true, SeqCst);
                    }
                }

                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    log::warn!("Robot worker receiver disconnected");
                    return;
                }
            }
            if let Some(port) = port.as_mut() {
                match port.bytes_to_read() {
                    Ok(0) => {}
                    Ok(_) => {
                        let mut byte = [0];
                        port.read(&mut byte).unwrap();
                        log::info!("{}", byte[0]);
                        busy.store(false, SeqCst);
                    }
                    Err(e) => log::error!("{e:?}"),
                }
            }
        }
    })
}
