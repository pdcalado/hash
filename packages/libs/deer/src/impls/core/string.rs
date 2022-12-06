use alloc::string::String;
use core::marker::PhantomData;

use error_stack::ResultExt;

use crate::{
    error::{DeserializeError, VisitorError},
    Deserialize, Deserializer, Document, Reflection, Schema, Visitor,
};

struct StrVisitor<'a>(PhantomData<fn() -> &'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for StrVisitor<'a> {
    type Value = &'a str;

    fn expecting(&self) -> Document {
        Document::new::<str>()
    }

    fn visit_borrowed_str(self, v: &'de str) -> error_stack::Result<Self::Value, VisitorError> {
        Ok(v)
    }
}

impl<'a> Reflection for &'a str {
    fn schema(_: &mut Document) -> Schema {
        Schema::new("string")
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a str {
    fn deserialize<D: Deserializer<'de>>(de: D) -> error_stack::Result<Self, DeserializeError> {
        de.deserialize_str(StrVisitor(Default::default()))
            .change_context(DeserializeError)
    }
}

struct CharVisitor;

impl<'de> Visitor<'de> for CharVisitor {
    type Value = char;

    fn expecting(&self) -> Document {
        Document::new::<char>()
    }

    fn visit_char(self, v: char) -> error_stack::Result<Self::Value, VisitorError> {
        Ok(v)
    }
}

impl Reflection for char {
    fn schema(_: &mut Document) -> Schema {
        Schema::new("string")
            .with("minLength", 1)
            .with("maxLength", 1)
    }
}

impl<'de> Deserialize<'de> for char {
    fn deserialize<D: Deserializer<'de>>(de: D) -> error_stack::Result<Self, DeserializeError> {
        de.deserialize_char(CharVisitor)
            .change_context(DeserializeError)
    }
}
