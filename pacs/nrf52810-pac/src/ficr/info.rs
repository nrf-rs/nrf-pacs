#[repr(C)]
#[doc = "Device info"]
#[doc(alias = "INFO")]
pub struct Info {
    part: Part,
    variant: Variant,
    package: Package,
    ram: Ram,
    flash: Flash,
}
impl Info {
    #[doc = "0x00 - Part code"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
    #[doc = "0x04 - Part variant, hardware version and production configuration"]
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
}
#[doc = "PART (r) register accessor: Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`] module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: Part variant, hardware version and production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@variant`] module"]
#[doc(alias = "VARIANT")]
pub type Variant = crate::Reg<variant::VariantSpec>;
#[doc = "Part variant, hardware version and production configuration"]
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
