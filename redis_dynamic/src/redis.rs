use std::ptr;
use super::structs::redisClient;

pub type Client = *const redisClient;

pub struct Handle {
    client: Client
}

impl Handle {
    pub fn new(client: Client) -> Handle {
        Handle {
            client: client
        }
    }

    pub fn args(&self) -> Vec<Vec<u8>> {
        args(self.client)
    }

    pub fn add_reply(&self, reply: &str) {
        unsafe {
            let reply = format!("{}\r\n\0", reply);
            let bytes = reply.as_bytes();
            addReplyString(self.client, bytes.as_ptr(), bytes.len() - 1);
        }
    }
    pub fn add_reply_bytes(&self, reply: &[u8]) {
        unsafe {
            addReplyString(self.client,
                           reply as *const _ as *const u8,
                           reply.len());
            addReplyString(self.client, b"\r\n\0" as *const u8, 2);
        }
    }

    pub fn error_reply(&self, reply: &str) {
        self.add_reply(&format!("-{}", reply));
    }
    pub fn status_reply(&self, reply: &str) {
        self.add_reply(&format!("+{}", reply));
    }
    pub fn integer_reply(&self, reply: i64) {
        self.add_reply(&format!(":{}", reply));
    }
    pub fn ok_reply(&self) {
        self.add_reply("+OK");
    }
}

extern {
    fn addReplyString(client: Client, s: *const u8, len: usize);
}

unsafe fn sds_to_vec(s: *const u8) -> Vec<u8> {
    let start = s.offset(-8);
    let len = *(start as *const u32) as usize;
    let mut dst = Vec::with_capacity(len);

    ptr::copy_nonoverlapping(s, dst.as_mut_ptr(), len);
    dst.set_len(len);
    dst
}

pub fn args(client: Client) -> Vec<Vec<u8>> {
    let argc = unsafe { (*client).argc } as isize;
    let argv = unsafe { (*client).argv };

    let mut args = Vec::new();
    for i in 0..argc {
        unsafe {
            let arg_ptr = argv.offset(i);
            let obj_ptr = *arg_ptr;
            let char_ptr = (*obj_ptr).ptr as *const u8;

            let v = sds_to_vec(char_ptr);
            args.push(v);
        }
    }

    args
}
