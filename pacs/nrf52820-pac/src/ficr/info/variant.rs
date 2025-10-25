#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Build code (hardware version and production configuration). Encoded as ASCII.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    #[doc = "1094795585: AAAA"]
    Aaaa = 1094795585,
    #[doc = "1094795843: AABC"]
    Aabc = 1094795843,
    #[doc = "1094796080: AAC0"]
    Aac0 = 1094796080,
    #[doc = "1094796081: AAC1"]
    Aac1 = 1094796081,
    #[doc = "1094796336: AAD0"]
    Aad0 = 1094796336,
    #[doc = "4294967295: Unspecified"]
    Unspecified = 4294967295,
}
impl From<Variant> for u32 {
    #[inline(always)]
    fn from(variant: Variant) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Variant {
    type Ux = u32;
}
impl crate::IsEnum for Variant {}
#[doc = "Field `VARIANT` reader - Build code (hardware version and production configuration). Encoded as ASCII."]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            1094795585 => Some(Variant::Aaaa),
            1094795843 => Some(Variant::Aabc),
            1094796080 => Some(Variant::Aac0),
            1094796081 => Some(Variant::Aac1),
            1094796336 => Some(Variant::Aad0),
            4294967295 => Some(Variant::Unspecified),
            _ => None,
        }
    }
    #[doc = "AAAA"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == Variant::Aaaa
    }
    #[doc = "AABC"]
    #[inline(always)]
    pub fn is_aabc(&self) -> bool {
        *self == Variant::Aabc
    }
    #[doc = "AAC0"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == Variant::Aac0
    }
    #[doc = "AAC1"]
    #[inline(always)]
    pub fn is_aac1(&self) -> bool {
        *self == Variant::Aac1
    }
    #[doc = "AAD0"]
    #[inline(always)]
    pub fn is_aad0(&self) -> bool {
        *self == Variant::Aad0
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Variant::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Build code (hardware version and production configuration)\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT to value 0xffff_ffff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
