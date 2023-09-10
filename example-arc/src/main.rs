use std::thread;

use ni_fpga::{ReadOnly, Register, RegisterReadAccess, SessionAccess, StoredOffset};

use crate::registers::FpgaBitfile;

mod registers;

fn main() -> Result<(), ni_fpga::Error> {
    let mut regs = FpgaBitfile::take().unwrap();

    let session = FpgaBitfile::session_builder("rio://172.22.11.2/RIO0")?.build_arc()?;

    let dc0 = regs.DutyCycle0_Frequency.take().unwrap();
    let r = dc0.read(&session);

    let dc1_src = regs.DutyCycle1_Source.take().unwrap();
    let configs = dc1_src.read(&session);

    let session_2 = session.clone();

    let voltage_register = unsafe { session.open_readonly_register::<u16>(99174) };
    let voltage_register_2 = unsafe { session.open_readonly_const_register::<u16, 99174>() };

    let voltage_register_3: Register<u16, ReadOnly, StoredOffset> =
        unsafe { session.open_readonly_const_register::<u16, 99174>() }.into();

    let read_pwm_thread = thread::spawn(move || voltage_register_2.read(&session_2));

    println!("Input voltage: {:?}", voltage_register.read(&session)?);
    println!("Input voltage: {:?}", voltage_register_3.read(&session)?);
    println!("Input voltage: {:?}", read_pwm_thread.join().unwrap()?);
    Ok(())
}
