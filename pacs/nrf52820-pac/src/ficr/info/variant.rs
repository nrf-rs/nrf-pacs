#[doc = "Register `VARIANT` reader"]
pub struct R(crate::R<VARIANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VARIANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VARIANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VARIANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Build code (hardware version and production configuration). Encoded as ASCII.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1094795843: AABC"]
    AABC = 1094795843,
    #[doc = "1094796080: AAC0"]
    AAC0 = 1094796080,
    #[doc = "1094796081: AAC1"]
    AAC1 = 1094796081,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VARIANT` reader - Build code (hardware version and production configuration). Encoded as ASCII."]
pub struct VARIANT_R(crate::FieldReader<u32, VARIANT_A>);
impl VARIANT_R {
    pub(crate) fn new(bits: u32) -> Self {
        VARIANT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            1094795585 => Some(VARIANT_A::AAAA),
            1094795843 => Some(VARIANT_A::AABC),
            1094796080 => Some(VARIANT_A::AAC0),
            1094796081 => Some(VARIANT_A::AAC1),
            4294967295 => Some(VARIANT_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        **self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AABC`"]
    #[inline(always)]
    pub fn is_aabc(&self) -> bool {
        **self == VARIANT_A::AABC
    }
    #[doc = "Checks if the value of the field is `AAC0`"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        **self == VARIANT_A::AAC0
    }
    #[doc = "Checks if the value of the field is `AAC1`"]
    #[inline(always)]
    pub fn is_aac1(&self) -> bool {
        **self == VARIANT_A::AAC1
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == VARIANT_A::UNSPECIFIED
    }
}
impl core::ops::Deref for VARIANT_R {
    type Target = crate::FieldReader<u32, VARIANT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Build code (hardware version and production configuration)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](index.html) module"]
pub struct VARIANT_SPEC;
impl crate::RegisterSpec for VARIANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [variant::R](R) reader structure"]
impl crate::Readable for VARIANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VARIANT to value 0xffff_ffff"]
impl crate::Resettable for VARIANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
