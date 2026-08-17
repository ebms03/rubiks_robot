use crossbeam_channel::{Receiver, TryRecvError};
use protocol::{DesktopToArduinoPacket, encode_desktop_to_arduino_packet};
use std::{
    sync::{Arc, atomic::AtomicBool},
    thread,
};

pub enum RobotCommand {
    Shutdown,
    Packet(DesktopToArduinoPacket),
}

impl From<DesktopToArduinoPacket> for RobotCommand {
    fn from(value: DesktopToArduinoPacket) -> Self {
        Self::Packet(value)
    }
}

pub fn robot_worker(
    // port: Box<dyn serialport::SerialPort>,
    receiver: Receiver<RobotCommand>,
    busy: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match receiver.try_recv() {
                Ok(RobotCommand::Packet(p)) => {
                    let byte = encode_desktop_to_arduino_packet(p);
                }
                Ok(RobotCommand::Shutdown) => return,

                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    log::error!("Robot worker receiver disconnected");
                    return;
                }
            }
            // match port.bytes_to_read() {
            //     Ok(0) => {}
            //     Ok(n) => {}
            //     Err(e) => log::error!("{e:?}"),
            // }
        }
    })
}
