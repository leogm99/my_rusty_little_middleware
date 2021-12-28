const DEFINE: u8 = 0x64;
const PUSH: u8 = 0x75;
const POP: u8 = 0x6f;

pub trait CommandSerializer {
    fn serialize(&self) -> Vec<u8>;
}

impl CommandSerializer for DefineCommand {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(DEFINE);
        let mut queue_name = self.queue.as_bytes().to_vec();
        let mut qsize_be = queue_name.len().to_be_bytes()[4..8].to_vec();
        data.append(&mut qsize_be);
        data.append(&mut queue_name);
        data
    }
}

impl CommandSerializer for PushCommand {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(PUSH);
        data.append(&mut self.queue.as_bytes().to_vec());
        data.append(&mut self.message.as_bytes().to_vec());
        data
    }
}

impl CommandSerializer for PopCommand {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(POP);
        data.append(&mut self.queue.as_bytes().to_vec());
        data
    }
}