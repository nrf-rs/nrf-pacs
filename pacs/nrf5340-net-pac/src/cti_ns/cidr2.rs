#[doc = "Register `CIDR2` reader"]
pub struct R(crate::R<CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\]
of the component identification code.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMBL_2_A {
    #[doc = "5: Bits\\[23:16\\]
of the identification code."]
    VALUE = 5,
}
impl From<PRMBL_2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRMBL_2` reader - Preamble\\[2\\]. Contains bits\\[23:16\\]
of the component identification code."]
pub struct PRMBL_2_R(crate::FieldReader<u8, PRMBL_2_A>);
impl PRMBL_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRMBL_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRMBL_2_A> {
        match self.bits {
            5 => Some(PRMBL_2_A::VALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        **self == PRMBL_2_A::VALUE
    }
}
impl core::ops::Deref for PRMBL_2_R {
    type Target = crate::FieldReader<u8, PRMBL_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[2\\]. Contains bits\\[23:16\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr2](index.html) module"]
pub struct CIDR2_SPEC;
impl crate::RegisterSpec for CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr2::R](R) reader structure"]
impl crate::Readable for CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for CIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
