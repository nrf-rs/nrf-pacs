#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "KEY")]
pub struct Key {
    value: [Value; 4],
}
impl Key {
    #[doc = "0x00..0x10 - Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
    #[inline(always)]
    pub const fn value(&self, n: usize) -> &Value {
        &self.value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
    #[inline(always)]
    pub fn value_iter(&self) -> impl Iterator<Item = &Value> {
        self.value.iter()
    }
}
#[doc = "VALUE (rw) register accessor: Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
pub mod value;
