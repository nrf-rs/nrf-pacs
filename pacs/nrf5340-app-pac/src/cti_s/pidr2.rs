#[doc = "Register `PIDR2` reader"]
pub struct R(crate::R<PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DES_1_A {
    #[doc = "3: Arm. Bits\\[6:4\\]
of the JEDEC JEP106 Identity Code"]
    ARM = 3,
}
impl From<DES_1_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DES_1` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub struct DES_1_R(crate::FieldReader<u8, DES_1_A>);
impl DES_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DES_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DES_1_A> {
        match self.bits {
            3 => Some(DES_1_A::ARM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ARM`"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        **self == DES_1_A::ARM
    }
}
impl core::ops::Deref for DES_1_R {
    type Target = crate::FieldReader<u8, DES_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEDEC` reader - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
pub struct JEDEC_R(crate::FieldReader<bool, bool>);
impl JEDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Peripheral revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVISION_A {
    #[doc = "0: This device is at r0p0"]
    REV0P0 = 0,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REVISION` reader - Peripheral revision"]
pub struct REVISION_R(crate::FieldReader<u8, REVISION_A>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVISION_A> {
        match self.bits {
            0 => Some(REVISION_A::REV0P0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REV0P0`"]
    #[inline(always)]
    pub fn is_rev0p0(&self) -> bool {
        **self == REVISION_A::REV0P0
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, REVISION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](index.html) module"]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr2::R](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for PIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
