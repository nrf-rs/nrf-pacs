#[repr(C)]
#[doc = "Device info"]
#[doc(alias = "INFO")]
pub struct Info {
    part: Part,
    variant: Variant,
    package: Package,
    ram: Ram,
    flash: Flash,
    unused0: [Unused0; 3],
}
impl Info {
    #[doc = "0x00 - Part code"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
    #[doc = "0x04 - Part Variant, Hardware version and Production configuration"]
    #[inline(always)]
    pub const fn variant(&self) -> &Variant {
        &self.variant
    }
    #[doc = "0x08 - Package option"]
    #[inline(always)]
    pub const fn package(&self) -> &Package {
        &self.package
    }
    #[doc = "0x0c - RAM variant"]
    #[inline(always)]
    pub const fn ram(&self) -> &Ram {
        &self.ram
    }
    #[doc = "0x10 - Flash variant"]
    #[inline(always)]
    pub const fn flash(&self) -> &Flash {
        &self.flash
    }
    #[doc = "0x14..0x20 - Description collection\\[0\\]: Unspecified"]
    #[inline(always)]
    pub const fn unused0(&self, n: usize) -> &Unused0 {
        &self.unused0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x20 - Description collection\\[0\\]: Unspecified"]
    #[inline(always)]
    pub fn unused0_iter(&self) -> impl Iterator<Item = &Unused0> {
        self.unused0.iter()
    }
}
#[doc = "PART (r) register accessor: Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`] module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@variant`] module"]
#[doc(alias = "VARIANT")]
pub type Variant = crate::Reg<variant::VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "PACKAGE (r) register accessor: Package option\n\nYou can [`read`](crate::Reg::read) this register and get [`package::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@package`] module"]
#[doc(alias = "PACKAGE")]
pub type Package = crate::Reg<package::PackageSpec>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM (r) register accessor: RAM variant\n\nYou can [`read`](crate::Reg::read) this register and get [`ram::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram`] module"]
#[doc(alias = "RAM")]
pub type Ram = crate::Reg<ram::RamSpec>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "FLASH (r) register accessor: Flash variant\n\nYou can [`read`](crate::Reg::read) this register and get [`flash::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash`] module"]
#[doc(alias = "FLASH")]
pub type Flash = crate::Reg<flash::FlashSpec>;
#[doc = "Flash variant"]
pub mod flash;
#[doc = "UNUSED0 (rw) register accessor: Description collection\\[0\\]: Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unused0`] module"]
#[doc(alias = "UNUSED0")]
pub type Unused0 = crate::Reg<unused0::Unused0Spec>;
#[doc = "Description collection\\[0\\]: Unspecified"]
pub mod unused0;
