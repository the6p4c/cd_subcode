use std::env;
use std::fs;
use std::io::{self, Read};

mod channel_q;
mod subcode;

use channel_q::ChannelQ;
use subcode::SubcodeSymbol;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let subcode_dump_filename = args
        .get(1)
        .expect("first argument should be subcode dump filename");

    let mut subcode_dump = fs::File::open(subcode_dump_filename)?;

    let mut frame = [0u8; 96];
    while subcode_dump.read(&mut frame)? == 96 {
        let frame_symbols = frame.iter().map(|b| SubcodeSymbol(*b)).collect::<Vec<_>>();

        let channel_q = ChannelQ::new(&frame_symbols);
        println!(
            "control: {:#06b}, address: {:?}, crc: {:#018b}",
            channel_q.control(),
            channel_q.adr(),
            channel_q.crc()
        );

        if let Some(mode1) = channel_q.as_mode1() {
            println!("\ttrack number: {}", mode1.tno());

            println!(
                "\ttrack running time: {:02}:{:02}:{:02} (mm:ss:ff)",
                mode1.min(),
                mode1.sec(),
                mode1.frame()
            );

            // track number is 0 during lead-in track
            if mode1.tno() != 0 {
                println!(
                    "\tdisc running time:  {:02}:{:02}:{:02} (mm:ss:ff)",
                    mode1.a_p_min(),
                    mode1.a_p_sec(),
                    mode1.a_p_frame()
                );
            } else {
                println!(
                    "\tpoint = {}, track running time {:02}:{:02}:{:02} (mm:ss:ff)",
                    mode1.point(),
                    mode1.a_p_min(),
                    mode1.a_p_sec(),
                    mode1.a_p_frame()
                );
            }
        } else if let Some(mode2) = channel_q.as_mode2() {
            println!("\tcatalogue number: {}", mode2.catalogue_number());
        } else if let Some(mode3) = channel_q.as_mode3() {
            println!("\tisrc: {}", mode3.isrc());
            println!("\t\tcountry code: {}", mode3.country_code());
            println!("\t\towner code: {}", mode3.owner_code());
            println!("\t\tyear: {}", mode3.year());
            println!("\t\tserial number: {}", mode3.serial_number());
        }
    }

    Ok(())
}
