use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

// NBT Format description: https://minecraft.fandom.com/wiki/NBT_format

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: Option<String>,
    pub payload: TagPayload,
}

impl Tag {
    fn new(header: TagHeader, payload: TagPayload) -> Self {
        return Tag {
            name: header.name,
            payload,
        };
    }
}

#[derive(Debug, Clone)]
pub enum TagPayload {
    Compound(Vec<Tag>),
    IntArray(Vec<i32>),
    ByteArray(Vec<i8>),
    List(Vec<TagPayload>),

    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),

    String(String),

    Unknown,
}

#[derive(Debug)]
struct TagHeader {
    kind: i8,
    name: Option<String>,
}

pub struct TagReader<'a, R: io::Read> {
    reader: &'a mut R,
}

impl<'a, R: io::Read> TagReader<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self { reader }
    }

    pub fn read_tag(&mut self) -> Result<Tag, io::Error> {
        let header = self.read_header()?;
        let payload = self.read_payload(header.kind)?;

        Ok(Tag::new(header, payload))
    }

    fn read_payload(&mut self, kind: i8) -> Result<TagPayload, io::Error> {
        Ok(match kind {
            1 => TagPayload::Byte(self.reader.read_i8()?),
            2 => TagPayload::Short(self.reader.read_i16::<LittleEndian>()?),
            3 => TagPayload::Int(self.reader.read_i32::<LittleEndian>()?),
            4 => TagPayload::Long(self.reader.read_i64::<LittleEndian>()?),
            5 => TagPayload::Float(self.reader.read_f32::<LittleEndian>()?),
            6 => TagPayload::Double(self.reader.read_f64::<LittleEndian>()?),
            7 => TagPayload::ByteArray(self.read_i8_array()?),
            8 => TagPayload::String(self.read_string()?),
            9 => TagPayload::List(self.read_list()?),
            10 => TagPayload::Compound(self.read_compound()?),
            11 => TagPayload::IntArray(self.read_i32_array()?),
            _ => TagPayload::Unknown,
        })
    }

    fn read_compound(&mut self) -> Result<Vec<Tag>, io::Error> {
        let mut result = Vec::new();
        loop {
            let header = self.read_header()?;
            if header.kind == 0 {
                break;
            }

            let payload = self.read_payload(header.kind)?;
            result.push(Tag::new(header, payload));
        }
        Ok(result)
    }

    fn read_list(&mut self) -> Result<Vec<TagPayload>, io::Error> {
        let kind = self.reader.read_i8()?;
        let length = self.reader.read_i32::<LittleEndian>()?;
        let mut vec = vec![TagPayload::Unknown; length as usize];
        for i in 0..length {
            vec[i as usize] = self.read_payload(kind)?;
        }
        Ok(vec)
    }

    fn read_i8_array(&mut self) -> Result<Vec<i8>, io::Error> {
        let length = self.reader.read_i32::<LittleEndian>()?;
        let mut vec = vec![0i8; length as usize];
        for i in 0..length {
            vec[i as usize] = self.reader.read_i8()?;
        }
        Ok(vec)
    }

    fn read_i32_array(&mut self) -> Result<Vec<i32>, io::Error> {
        let length = self.reader.read_i32::<LittleEndian>()?;
        let mut vec = vec![0i32; length as usize];
        for i in 0..length {
            vec[i as usize] = self.reader.read_i32::<LittleEndian>()?;
        }
        Ok(vec)
    }

    fn read_string(&mut self) -> Result<String, io::Error> {
        let string_length = self.reader.read_u16::<LittleEndian>()?;
        let mut string_buf = vec![0u8; string_length as usize];
        self.reader.read_exact(&mut string_buf)?;
        return Ok(String::from_utf8_lossy(&string_buf).to_string());
    }

    fn read_header(&mut self) -> Result<TagHeader, io::Error> {
        let kind = self.reader.read_i8()?;
        if kind == 0 {
            Ok(TagHeader { kind, name: None })
        } else {
            let name = self.read_string()?;
            Ok(TagHeader {
                kind,
                name: Some(name),
            })
        }
    }
}
