#[doc = "Register `CIDR3` reader"]
pub struct R(crate::R<CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_3` reader - Preamble\\[3\\]. Contains bits\\[31:24\\]
of the component identification code."]
pub type PRMBL_3_R = crate::FieldReader<u8, PRMBL_3_A>;
#[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\]
of the component identification code.\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMBL_3_A {
    #[doc = "177: Bits\\[31:24\\]
of the identification code."]
    VALUE = 177,
}
impl From<PRMBL_3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_3_A) -> Self {
        variant as _
    }
}
impl PRMBL_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRMBL_3_A> {
        match self.bits {
            177 => Some(PRMBL_3_A::VALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_3_A::VALUE
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[3\\]. Contains bits\\[31:24\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr3](index.html) module"]
pub struct CIDR3_SPEC;
impl crate::RegisterSpec for CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr3::R](R) reader structure"]
impl crate::Readable for CIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for CIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
