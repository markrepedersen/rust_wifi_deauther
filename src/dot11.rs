use crate::frame::Addr;
use crate::packet::Payload;
use crate::parse::{self, BitParsable};

use custom_debug_derive::*;
use nom::{
    bits::bits, bytes::complete::take, error::context, number::complete::be_u16, sequence::tuple,
};
use ux::*;

#[derive(CustomDebug)]
pub struct FrameControl {
    #[debug(skip)]
    pub version: u2,

    #[debug(format = "{:b}")]
    pub ftype: u2,

    #[debug(format = "{}")]
    pub subtype: u4,

    #[debug(format = "{:b}")]
    pub to_ds: u1,

    #[debug(format = "{:b}")]
    pub from_ds: u1,

    #[debug(format = "{:b}")]
    pub more_fragments: u1,

    #[debug(format = "{:b}")]
    pub retry: u1,

    #[debug(format = "{:b}")]
    pub power_mgmt: u1,

    #[debug(format = "{:b}")]
    pub more_data: u1,

    #[debug(format = "{:b}")]
    pub wep: u1,

    #[debug(format = "{:b}")]
    pub order: u1,
}

impl FrameControl {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        let (i, (version, ftype, subtype)) = bits(tuple((u2::parse, u2::parse, u4::parse)))(i)?;
        let (i, (to_ds, from_ds, more_fragments, retry, power_mgmt, more_data, wep, order)) =
            bits(tuple((
                u1::parse,
                u1::parse,
                u1::parse,
                u1::parse,
                u1::parse,
                u1::parse,
                u1::parse,
                u1::parse,
            )))(i)?;
        let res = Self {
            version,
            ftype,
            subtype,
            to_ds,
            from_ds,
            more_fragments,
            retry,
            power_mgmt,
            more_data,
            wep,
            order,
        };
        return Ok((i, res));
    }
}

#[derive(CustomDebug)]
pub struct Dot11Frame {
    pub fc: FrameControl,
    pub duration: u16,
    pub addr1: Addr,
    pub addr2: Addr,
    pub addr3: Addr,
    pub addr4: Addr,
    pub seq_control: u16,
    pub payload: Payload,
    pub crc: u16,
}

impl Dot11Frame {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("802.11 MAC frame", |i| {
            let (i, fc) = FrameControl::parse(i)?;
            let (i, duration) = be_u16(i)?;
            let (i, (addr1, addr2, addr3, addr4)) =
                tuple((Addr::parse, Addr::parse, Addr::parse, Addr::parse))(i)?;
            let (i, seq_control) = be_u16(i)?;
            let (i, _) = take(i.len() - 2)(i)?;
            let (i, crc) = be_u16(i)?;
            let res = Self {
                fc,
                duration,
                addr1,
                addr2,
                addr3,
                addr4,
                seq_control,
                payload: Payload::Unknown,
                crc,
            };
            Ok((i, res))
        })(i)
    }

    pub fn set_fc(mut self, fc: FrameControl) -> Self {
        self.fc = fc;
        return self;
    }

    pub fn set_addrs(mut self, addr1: Addr, addr2: Addr, addr3: Addr, addr4: Addr) -> Self {
        self.addr1 = addr1;
        self.addr2 = addr2;
        self.addr3 = addr3;
        self.addr4 = addr4;
        return self;
    }

    pub fn set_duration(mut self, duration: u16) -> Self {
        self.duration = duration;
        return self;
    }

    pub fn set_seq_control(mut self, seq_control: u16) -> Self {
        self.seq_control = seq_control;
        return self;
    }

    pub fn set_crc(mut self, crc: u16) -> Self {
        self.crc = crc;
        return self;
    }

    pub fn new(
        fc: FrameControl,
        duration: u16,
        addr1: Addr,
        addr2: Addr,
        addr3: Addr,
        addr4: Addr,
        seq_control: u16,
        payload: Payload,
        crc: u16,
    ) -> Self {
        return Self {
            fc,
            duration,
            addr1,
            addr2,
            addr3,
            addr4,
            seq_control,
            payload,
            crc,
        };
    }
}
