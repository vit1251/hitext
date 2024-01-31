
use nom::number::streaming::{be_u16, be_u32, be_u8};
use nom::IResult;

#[repr(u8)]
#[derive(Debug)]
pub enum MsgType {
    Reset = 0,
    Write = 1,
    SetColor = 2,
    GotoXY = 3,
}

impl From<u8> for MsgType {
    fn from(item: u8) -> Self {
        match item {
            0 => MsgType::Reset,
            1 => MsgType::Write,
            2 => MsgType::SetColor,
            3 => MsgType::GotoXY,
            _ => unreachable!(),
        }
    }
}

pub fn take_msg_type(input: &[u8]) -> IResult<&[u8], MsgType> {
    let (input, msg_type) = be_u8(input)?;
    let mt: MsgType = MsgType::from(msg_type);
    Ok((input, mt))
}

pub fn take_char(input: &[u8]) -> IResult<&[u8], char> {
    let (input, c) = be_u8(input)?;
    let c = c as char;
    Ok((input, c))
}

pub fn take_color(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, color) = be_u8(input)?;
    Ok((input, color))
}

pub fn take_pos(input: &[u8]) -> IResult<&[u8], u16> {
    let (input, pos) = be_u16(input)?;
    Ok((input, pos))
}

#[derive(Debug)]
pub enum Msg {
    Reset,              // 0x00
    Write(char),        // 0x01
    SetColor(u8,u8,u8), // 0x02
    GotoXY(u16, u16),   // 0x03
}

pub fn parse_msg(input: &[u8]) -> (&[u8], Msg) {
    let (input, msg_type) = take_msg_type(input).unwrap();
    //println!("msg_type => {:?}", msg_type);
    let (input, msg) = match msg_type {
        MsgType::Reset => {
            (input, Msg::Reset)
        },
        MsgType::Write => {
            let (input, c) = take_char(input).unwrap();
            (input, Msg::Write(c))
        }
        MsgType::SetColor => {
            let (input, r) = take_color(input).unwrap();
            let (input, g) = take_color(input).unwrap();
            let (input, b) = take_color(input).unwrap();
            let (input, a) = take_color(input).unwrap();
            (input, Msg::SetColor(r,g,b))
        }
        MsgType::GotoXY => {
            let (input, x) = take_pos(input).unwrap();
            let (input, y) = take_pos(input).unwrap();
            (input, Msg::GotoXY(x, y))
        }
    };
    (input, msg)
}

pub fn parse(input: &[u8]) -> Vec<Msg> {
    let mut msgs: Vec<Msg> = Vec::<Msg>::new();
    let mut input: &[u8] = input;
    let mut msg: Msg;
    loop {
        let size = input.len();
        //println!("size = {}", size);
        if size == 0 {
            break;
        }
        //println!("input = {:?}", input);
        (input, msg) = parse_msg(input);
        //println!("msg = {:?}", msg);
        msgs.push(msg);
    }
    msgs
}
