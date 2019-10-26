pub trait CanSerializeNoConversion {
    fn as_str(&self) -> &str;
}

impl CanSerializeNoConversion for String {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl CanSerializeNoConversion for &String {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl CanSerializeNoConversion for str {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl CanSerializeNoConversion for Box<str> {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl CanSerializeNoConversion for Box<String> {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}

impl CanSerializeNoConversion for Box<&String> {
    #[inline]
    fn as_str(&self) -> &str {
        self
    }
}
