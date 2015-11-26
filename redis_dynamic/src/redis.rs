use std::ptr;
use super::structs::redisClient;

pub type Client = *const redisClient;

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
pub fn add_reply_bytes(client: Client, reply: &[u8]) {
    unsafe {
        addReplyString(client, reply as *const _ as *const u8, reply.len());
        addReplyString(client, b"\r\n\0" as *const u8, 2);
    }
}

pub fn error_reply(client: Client, reply: &str) {
    add_reply(client, &format!("-{}", reply));
}
pub fn status_reply(client: Client, reply: &str) {
    add_reply(client, &format!("+{}", reply));
}
pub fn integer_reply(client: Client, reply: i64) {
    add_reply(client, &format!(":{}", reply));
}
pub fn ok_reply(client: Client) {
    add_reply(client, "+OK");
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
