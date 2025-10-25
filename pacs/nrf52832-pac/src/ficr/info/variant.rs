#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII\n\nValue on reset: 1094795586"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    #[doc = "1094795585: AAAA"]
    Aaaa = 1094795585,
    #[doc = "1094795586: AAAB"]
    Aaab = 1094795586,
    #[doc = "1094795841: AABA"]
    Aaba = 1094795841,
    #[doc = "1094795842: AABB"]
    Aabb = 1094795842,
    #[doc = "1094795824: AAB0"]
    Aab0 = 1094795824,
    #[doc = "1094796592: AAE0"]
    Aae0 = 1094796592,
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
#[doc = "Field `VARIANT` reader - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            1094795585 => Some(Variant::Aaaa),
            1094795586 => Some(Variant::Aaab),
            1094795841 => Some(Variant::Aaba),
            1094795842 => Some(Variant::Aabb),
            1094795824 => Some(Variant::Aab0),
            1094796592 => Some(Variant::Aae0),
            4294967295 => Some(Variant::Unspecified),
            _ => None,
        }
    }
    #[doc = "AAAA"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == Variant::Aaaa
    }
    #[doc = "AAAB"]
    #[inline(always)]
    pub fn is_aaab(&self) -> bool {
        *self == Variant::Aaab
    }
    #[doc = "AABA"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == Variant::Aaba
    }
    #[doc = "AABB"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == Variant::Aabb
    }
    #[doc = "AAB0"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == Variant::Aab0
    }
    #[doc = "AAE0"]
    #[inline(always)]
    pub fn is_aae0(&self) -> bool {
        *self == Variant::Aae0
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Variant::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT to value 0x4141_4142"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u32 = 0x4141_4142;
}
