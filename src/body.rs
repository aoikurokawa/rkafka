pub trait Body {
    fn to_bytes(&self) -> Vec<u8>;

    fn size(&self) -> u32;
}
