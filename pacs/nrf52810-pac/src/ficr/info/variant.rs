#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Part variant, hardware version and production configuration, encoded as ASCII\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    #[doc = "1094795585: AAAA"]
    Aaaa = 1094795585,
    #[doc = "1094795568: AAA0"]
    Aaa0 = 1094795568,
    #[doc = "1094795841: AABA"]
    Aaba = 1094795841,
    #[doc = "1094795842: AABB"]
    Aabb = 1094795842,
    #[doc = "1094795824: AAB0"]
    Aab0 = 1094795824,
    #[doc = "1094796097: AACA"]
    Aaca = 1094796097,
    #[doc = "1094796098: AACB"]
    Aacb = 1094796098,
    #[doc = "1094796080: AAC0"]
    Aac0 = 1094796080,
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
#[doc = "Field `VARIANT` reader - Part variant, hardware version and production configuration, encoded as ASCII"]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            1094795585 => Some(Variant::Aaaa),
            1094795568 => Some(Variant::Aaa0),
            1094795841 => Some(Variant::Aaba),
            1094795842 => Some(Variant::Aabb),
            1094795824 => Some(Variant::Aab0),
            1094796097 => Some(Variant::Aaca),
            1094796098 => Some(Variant::Aacb),
            1094796080 => Some(Variant::Aac0),
            4294967295 => Some(Variant::Unspecified),
            _ => None,
        }
    }
    #[doc = "AAAA"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == Variant::Aaaa
    }
    #[doc = "AAA0"]
    #[inline(always)]
    pub fn is_aaa0(&self) -> bool {
        *self == Variant::Aaa0
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
    #[doc = "AACA"]
    #[inline(always)]
    pub fn is_aaca(&self) -> bool {
        *self == Variant::Aaca
    }
    #[doc = "AACB"]
    #[inline(always)]
    pub fn is_aacb(&self) -> bool {
        *self == Variant::Aacb
    }
    #[doc = "AAC0"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == Variant::Aac0
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == Variant::Unspecified
    }
}
impl R {
    #[doc = "Bits 0:31 - Part variant, hardware version and production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Part variant, hardware version and production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
