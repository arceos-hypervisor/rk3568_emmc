/// cmd
pub struct Cmd {
    idx: u32, // 寄存器基地址
    ctype: u32,
    resp: u32, // 响应寄存器基地址
}

impl Cmd {
    pub fn new(idx: u32, ctype: u32, resp: u32) -> Self {
        Cmd {
            idx,
            ctype,
            resp,
        }
    }

}
