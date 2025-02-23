use apache_avro::{Codec, Schema, Writer};
use serde::Serialize;

pub fn writer_from_schema(schema: &Schema) -> Writer<'_, Vec<u8>> {
	Writer::with_codec(schema, Vec::new(), Codec::Deflate)
}

pub fn serialize_avro<S: Serialize>(
	schema: &Schema,
	value: S,
) -> Result<Vec<u8>, apache_avro::Error> {
	let mut writer = writer_from_schema(schema);
	writer.append_ser(value)?;

	writer.into_inner()
}
