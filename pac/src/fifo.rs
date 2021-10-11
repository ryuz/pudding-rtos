use core::ptr;

pub struct Fifo {
    buf_size: u32,
    tx_ptr: u32,
    rx_ptr: u32,
    buf: [u8; 1],
}

impl Fifo {
    pub fn reset(&mut self, size: isize) {
        let buf_size = (size - 4*3) as u32;
        unsafe {
            ptr::write_volatile(&mut self.buf_size, 0);
            ptr::write_volatile(&mut self.tx_ptr, 0);
            ptr::write_volatile(&mut self.rx_ptr, 0);
            ptr::write_volatile(&mut self.buf_size, buf_size);
        }
    }
    
    pub fn send_char(&mut self, c: u8) -> bool {
        unsafe {
            let rx_ptr = ptr::read_volatile(&self.rx_ptr);
            let tx_ptr = ptr::read_volatile(&self.tx_ptr as *const u32);
            let buf = &mut self.buf[0] as *mut u8;
            let next_ptr = if tx_ptr + 1 < self.buf_size {
                tx_ptr + 1
            } else {
                0
            };
            if next_ptr != rx_ptr {
                ptr::write_volatile(buf.offset(tx_ptr as isize), c);
                ptr::write_volatile(&mut self.tx_ptr, next_ptr);
                true
            } else {
                false
            }
        }
    }

    pub fn recv_char(&mut self) -> Option<u8> {
        unsafe {
            let tx_ptr = ptr::read_volatile(&self.tx_ptr as *const u32);
            let rx_ptr = ptr::read_volatile(&self.rx_ptr as *const u32);
            let buf = &self.buf[0] as *const u8;
            if tx_ptr != rx_ptr {
                let c = ptr::read_volatile(buf.offset(rx_ptr as isize));
                let next_ptr = if rx_ptr + 1 < self.buf_size {
                    rx_ptr + 1
                } else {
                    0
                };
                ptr::write_volatile(&mut self.rx_ptr as *mut u32, next_ptr);
                Some(c)
            } else {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_test() {
        let mut mem :  [u8; 256] = [0; 256];
        let ptr = &mut mem as *mut u8;
        unsafe {
            let fifo = &mut *(ptr as *mut Fifo);
            fifo.buf_size = 3;
            fifo.tx_ptr = 0;
            fifo.rx_ptr = 0;
            assert_eq!(fifo.recv_char(), None);
            assert_eq!(fifo.send_char('a' as u8), true);
            assert_eq!(fifo.send_char('b' as u8), true);
            assert_eq!(fifo.send_char('c' as u8), false);
            assert_eq!(fifo.recv_char().unwrap(), 'a' as u8);
            assert_eq!(fifo.send_char('c' as u8), true);
            assert_eq!(fifo.send_char('d' as u8), false);
            assert_eq!(fifo.recv_char().unwrap(), 'b' as u8);
            assert_eq!(fifo.recv_char().unwrap(), 'c' as u8);
            assert_eq!(fifo.recv_char(), None);
        }
    }
}
