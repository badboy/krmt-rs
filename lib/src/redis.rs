use libc::c_void;

pub type Client = *const c_void;

extern {
    fn addReplyString(client: Client, s: *const u8, len: usize);
}

pub fn add_reply(client: Client, reply: &str) {
    unsafe {
        let reply = format!("{}\r\n\0", reply);
        let bytes = reply.as_bytes();
        addReplyString(client, bytes.as_ptr(), bytes.len() - 1);
    }
}