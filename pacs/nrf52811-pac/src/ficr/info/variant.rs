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
#[doc = "Field `VARIANT` reader - Part variant, hardware version and production configuration, encoded as ASCII"]
pub type VARIANT_R = crate::FieldReader<u32, VARIANT_A>;
#[doc = "Part variant, hardware version and production configuration, encoded as ASCII\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1094795568: AAA0"]
    AAA0 = 1094795568,
    #[doc = "1094795841: AABA"]
    AABA = 1094795841,
    #[doc = "1094795842: AABB"]
    AABB = 1094795842,
    #[doc = "1094795824: AAB0"]
    AAB0 = 1094795824,
    #[doc = "1094796097: AACA"]
    AACA = 1094796097,
    #[doc = "1094796098: AACB"]
    AACB = 1094796098,
    #[doc = "1094796080: AAC0"]
    AAC0 = 1094796080,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
impl VARIANT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            1094795585 => Some(VARIANT_A::AAAA),
            1094795568 => Some(VARIANT_A::AAA0),
            1094795841 => Some(VARIANT_A::AABA),
            1094795842 => Some(VARIANT_A::AABB),
            1094795824 => Some(VARIANT_A::AAB0),
            1094796097 => Some(VARIANT_A::AACA),
            1094796098 => Some(VARIANT_A::AACB),
            1094796080 => Some(VARIANT_A::AAC0),
            4294967295 => Some(VARIANT_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AAA0`"]
    #[inline(always)]
    pub fn is_aaa0(&self) -> bool {
        *self == VARIANT_A::AAA0
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AAB0`"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == VARIANT_A::AAB0
    }
    #[doc = "Checks if the value of the field is `AACA`"]
    #[inline(always)]
    pub fn is_aaca(&self) -> bool {
        *self == VARIANT_A::AACA
    }
    #[doc = "Checks if the value of the field is `AACB`"]
    #[inline(always)]
    pub fn is_aacb(&self) -> bool {
        *self == VARIANT_A::AACB
    }
    #[doc = "Checks if the value of the field is `AAC0`"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == VARIANT_A::AAC0
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part variant, hardware version and production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(self.bits)
    }
}
#[doc = "Part variant, hardware version and production configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](index.html) module"]
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
