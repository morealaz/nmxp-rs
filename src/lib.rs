//! This library provides neccessery data structures and functions to work
//! with Nanometrics Inc. Nanometrics Protocol (NP) and
//! NMXP seismic data packet format protocols.
//!
//! # Conventions and terminology
//! All numeric values are unsigned integers and big-endian ordered unless
//! stated otherwise.
//! - Bytes are 9 bits.
//! - Bytes are packed without any hidden bytes inserted for alignment.
//! - A "packet" refers to a single NP packet.
//! - Each time-series packet will contain one data "payload".
//! - A "band" refers to a data channel.
//!
//! # Overview of NP packet format
//! Each NP data packet contains the following information:
//! - A BandId: identifies the stream of NP packets by datasource and band
//! name.
//! - Sequence Number: packet sequence within the scope of the packet’s
//! BandId.
//! - Metadata Sequence Number: identifies associated metadata packets.
//! - Time: the time the data in the packet was generated, in nanoseconds
//! since 1970 epoch.
//! - Geographic Location.
//! - Data payload.
//! Taurus embeds NP packets in UDP packets for transmission via IP.
//!
//! # Seismic data packets
//! A seismic data packet is sent when full or when a significant event occurs
//! (time change, configuration change, going to reboot) that causes it to be
//! flushed immediately.
//!
//! Values in this section, for example an Offset field or a specific value in
//! a field Description, are applicable to seismic data packets as created on
//! Taurus.

/// ## NP Packet Header
/// There is a single main header block for each packet.
pub struct NpHeader {
    /// ASCII characters `N` & `P` or `0x4E50`.
    pub np_version: [u8; 2],
    /// The total size of the packet;
    /// either `499` or `243` for seismic data packets, for 7 or 3 data frames
    /// respectively.
    pub packet_size: u16,
    /// The sequence number of this packet in the specified band.
    /// The sequence number starts at `0` and increments with every packet.
    pub sequence_number: i32,
    /// The sequence number identifying the metadata packet containing
    /// additional information for this band. A value of `-1` or `0xFFFFFFFF`
    /// indicates that there is no associated metadata packet.
    pub meta_sequence_number: i32,
    /// The time in nanoseconds from the epoch (`1970-01-01 00:00:00 UTC`) of
    /// the data which this packet contains.
    pub start_time: u64,
    /// The latitude in micro-degrees of the device producing the data found
    /// in the body of the packet.
    pub latitude: i32,
    /// The longitude in micro-degrees of the device producing the data found   
    /// in the body of the packet.
    pub longitude: i32,
    /// The altitude in meters of the device producing the data found
    /// in the body of the packet.
    pub altitude: i16,
    /// The URI of the device which originally produced the data;
    ///
    /// `0xE8 modelNum serialNum serialNum`
    ///
    /// where model numbers:
    ///
    /// `11` = Taurus, `13` = Trident 305
    pub data_source: [u8; 4],
    /// The URI of the band to which this packet belongs;
    ///
    /// `0x89` for "band/timeseries1/"
    ///
    /// `0x8B` for "band/timeseries2/"
    ///
    /// `0x8D` for "band/timeseries3/"
    pub band_name: u8,
    /// Set to `0x00` `0x00` for seismic data to indicate not used.
    pub packet_extension_block: i16,
}

//! ## Data payload
//! A seismic data payload is comprised of a fixed payload header block,
//! containing 2 payload header extensions, and a payload body.

/// ### Payload header
pub struct NpPayloadHeader {
    /// Indicates the total size of the payload including itself, the rest of
    /// the header including the extension block, and the data;
    /// `206` or `462`, depending on payload body size.
    pub payload_size: u16,
    /// Set to `0x00` to indicate not used as there is only one payload for a
    /// seismic data packet
    pub payload_name: u8,
    /// Indicates the media type of data in the payload; for seismic data is
    /// Steim1 Encoded Time-Series
    ///
    /// `0x83`
    /// "application/dns.ca.nmx.steim1"
    pub payload_media_type: u8,
    /// Provides a means of adding additional payload-specific fields to a
    /// payload;
    ///
    /// `0x00 0x08`
    pub payload_extension_block: i16,
    /// Number of samples in this packet
    ///
    /// `0x05 0x87 numSamples numSamples`
    ///
    /// "http://nmx.ca/04/NP/numSamples"
    pub number_samples: [u8; 4],
    /// Sample rate used for data in this packet
    ///
    /// `0x05 0x85 sampleRate sampleRate`
    ///
    /// "http://nmx.ca/04/NP/sampleRate"
    pub sample_rate: [u8; 4],
}

//! ## Payload body
//! The seismic data payload body consists of 3 or 7 frames of Steim1
//! compressed data (without the 64 byte space for standard SEED headers) as
//! described in Appendix B of 
//! [the SEED manual](http://www.iris.edu/manuals/SEEDManual_V2.4.pdf).
//! The receiving application must add the standard headers to produce a valid
//! MiniSEED data record. The seismic data payload body starts at offset of 51.

pub struct NpPayload {
    pub header: NpPayloadHeader,
    pub body: Vec<u8>,
}

pub struct NpPacket {
    pub header: NpHeader,
    pub payload: NpPayload,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
