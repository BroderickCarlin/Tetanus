use defmt::Format;
use nom::{
    branch::alt,
    bytes::complete::tag,
    number::complete::{le_u16, le_u32, le_u8},
    IResult,
};

const NUM_CONTROL_CHANNELS: usize = 14;

const PACKET_ID_BIND1: u8 = 0xBB;
const PACKET_ID_BIND2: u8 = 0xBC;
const PACKET_ID_STICKS: u8 = 0x58;
const PACKET_ID_FAILSAFE: u8 = 0x56;

#[derive(Debug, Format, Clone, PartialEq)]
pub enum TransmitterPacket {
    Sticks(SticksPacket),
    Bind(BindPacket),
}

impl TransmitterPacket {
    pub fn from_bytes(bytes: &[u8]) -> IResult<&[u8], Self> {
        alt((SticksPacket::from_bytes, BindPacket::from_bytes))(bytes)
    }
}

#[derive(Debug, Format, Clone, PartialEq)]
pub struct SticksPacket {
    transmitter_id: u32,
    receiver_id: u32,
    sticks: [u16; NUM_CONTROL_CHANNELS],
}

impl SticksPacket {
    pub fn from_bytes(bytes: &[u8]) -> IResult<&[u8], TransmitterPacket> {
        let (bytes, _) = tag(&[PACKET_ID_STICKS])(bytes)?;
        let (bytes, transmitter_id) = le_u32(bytes)?;
        let (mut bytes, receiver_id) = le_u32(bytes)?;
        let mut sticks = [0u16; NUM_CONTROL_CHANNELS];

        for i in 0..NUM_CONTROL_CHANNELS {
            (bytes, sticks[i]) = le_u16(bytes)?;
        }

        Ok((
            bytes,
            TransmitterPacket::Sticks(SticksPacket {
                transmitter_id,
                receiver_id,
                sticks,
            }),
        ))
    }
}

#[derive(Debug, Format, Clone, PartialEq)]
pub struct BindPacket {
    transmitter_id: u32,
    receiver_id: u32,
    stage: u8,
}

impl BindPacket {
    pub fn from_bytes(bytes: &[u8]) -> IResult<&[u8], TransmitterPacket> {
        let (bytes, _) = alt((tag(&[PACKET_ID_BIND1]), tag(&[PACKET_ID_BIND2])))(bytes)?;
        let (bytes, transmitter_id) = le_u32(bytes)?;
        let (bytes, receiver_id) = le_u32(bytes)?;
        let (bytes, stage) = le_u8(bytes)?;

        Ok((
            bytes,
            TransmitterPacket::Bind(BindPacket {
                transmitter_id,
                receiver_id,
                stage,
            }),
        ))
    }
}
