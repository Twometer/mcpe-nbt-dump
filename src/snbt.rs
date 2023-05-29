use crate::nbt::{Tag, TagPayload};

pub trait ToSnbt {
    fn to_snbt(&self, depth: usize) -> String;
}

impl ToSnbt for Tag {
    fn to_snbt(&self, depth: usize) -> String {
        if let Some(name) = &self.name {
            if !name.is_empty() {
                return format!(
                    "{}{}: {}",
                    depth_to_string(depth),
                    name,
                    self.payload.to_snbt(depth)
                );
            }
        }

        self.payload.to_snbt(depth)
    }
}

impl ToSnbt for TagPayload {
    fn to_snbt(&self, depth: usize) -> String {
        match &self {
            TagPayload::Compound(v) => format!(
                "{{\n{}\n{}}}",
                v.iter()
                    .map(|itm| itm.to_snbt(depth + 1))
                    .collect::<Vec<_>>()
                    .join(", \n"),
                depth_to_string(depth),
            ),
            TagPayload::List(v) => format!(
                "[\n{}{}\n{}]",
                depth_to_string(depth + 1),
                v.iter()
                    .map(|itm| itm.to_snbt(depth + 1))
                    .collect::<Vec<_>>()
                    .join(&format!(", \n{}", depth_to_string(depth + 1))),
                depth_to_string(depth),
            ),

            TagPayload::IntArray(v) => format!(
                "[I;{}]",
                v.iter()
                    .map(|itm| format!("{}", itm))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TagPayload::ByteArray(v) => format!(
                "[B;{}]",
                v.iter()
                    .map(|itm| format!("{}B", itm))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            TagPayload::Byte(v) => format!("{}B", v),
            TagPayload::Short(v) => format!("{}S", v),
            TagPayload::Int(v) => format!("{}", v),
            TagPayload::Long(v) => format!("{}L", v),
            TagPayload::Float(v) => format!("{}F", v),
            TagPayload::Double(v) => format!("{}D", v),

            TagPayload::String(str) => format!("\"{}\"", &str),
            TagPayload::Unknown => "unknown".to_string(),
        }
    }
}

fn depth_to_string(depth: usize) -> String {
    " ".repeat(depth * 4)
}
