use kclvm_error::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Message{
    pub msg_id: String,
    pub msg: String,
    pub source_code: String,
    pub pos: Position,
    pub arguments: Vec<String>,
}


impl Message {
    pub fn new( 
        msg_id: String,
        file: String,
        msg: String,
        source_code: String,
        pos: Position,
        arguments: Vec<String>,
    ) -> Self {
        Self{ msg_id, msg, source_code, pos, arguments }
    }
}

// impl From<Message> for String{
//     fn from(msg: Message) -> String{
//         let s = format!("{}:{}:{}:{}:{}\n{}\n{}\n^", 
//         msg.file, msg.pos.0, msg.pos.1, msg.msg_id, msg.msg,
//         msg.source_code,
//         " ".repeat(msg.pos.1 as usize - 1));
//         s
//     }
// }

// impl PartialEq for Message {
//     fn eq(&self, other: &Self) -> bool {
//         self.msg_id == other.msg_id &&
//         self.file == other.file &&
//         self.msg == other.msg &&
//         self.source_code == other.source_code &&
//         self.arguments == other.arguments
//     }
// }

// impl Eq for Message {}


// pub struct MSG {
//     pub id: &'static str,
//     pub short_info: &'static str,
//     pub long_info: &'static str,
// }
#[derive(Clone)]
pub struct MSG{
    pub id: String,
    pub short_info: String,
    pub long_info: String,
}

// impl Clone for MSG {
//     fn clone(&self) -> Self{
//         Self { id: self.id, short_info: self.short_info, long_info: self.long_info }
//     }
// }

