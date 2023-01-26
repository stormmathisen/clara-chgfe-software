    use std::io::{Error};
    use serialport::{self, SerialPort};
    use std::time::{Duration};


    pub fn setup_uart(path: &str, timeout: Duration, baud_rate: u32) -> Result<Box<dyn SerialPort>, Error> {
        let mut uart_fd = serialport::new(path, baud_rate).open()?;
        uart_fd.set_timeout(timeout)?;
        uart_fd.clear(serialport::ClearBuffer::All)?;
        Ok(uart_fd)
    }

    pub fn send_byte(fd: &mut Box<dyn SerialPort>, byte: &[u8; 1]) -> Result<usize, Error> {
        let written = fd.write(byte)?;
        Ok(written)
    }

    pub fn send_bytes(fd: &mut Box<dyn SerialPort>, bytes: &Vec<u8>) -> Result<usize, Error>{
        let written = fd.write(bytes)?;
        Ok(written)
    }

    pub fn receive_byte() -> Result<[u8; 1], Error> {
        let mut byte:[u8; 1] = [0];
        Ok(byte)
    }


    pub fn receive_bytes() -> Result<Vec<u8>, Error> {
        let mut bytes:Vec<u8> = Vec::new();
        Ok(bytes)
    }