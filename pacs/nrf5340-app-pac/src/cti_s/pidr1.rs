#[doc = "Register `PIDR1` reader"]
pub struct R(crate::R<PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_1` reader - Bits\\[11:8\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub type PART_1_R = crate::FieldReader<u8, PART_1_A>;
#[doc = "Bits\\[11:8\\]
of the 12-bit part number of the component. The designer of the component assigns this part number.\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PART_1_A {
    #[doc = "13: Indicates bits\\[11:8\\]
of the part number of the component."]
    PARTNUMBER_H = 13,
}
impl From<PART_1_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_1_A) -> Self {
        variant as _
    }
}
impl PART_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PART_1_A> {
        match self.bits {
            13 => Some(PART_1_A::PARTNUMBER_H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PARTNUMBER_H`"]
    #[inline(always)]
    pub fn is_partnumber_h(&self) -> bool {
        *self == PART_1_A::PARTNUMBER_H
    }
}
#[doc = "Field `DES_0` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type DES_0_R = crate::FieldReader<u8, DES_0_A>;
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DES_0_A {
    #[doc = "11: Arm. Bits\\[3:0\\]
of the JEDEC JEP106 Identity Code"]
    ARM = 11,
}
impl From<DES_0_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_0_A) -> Self {
        variant as _
    }
}
impl DES_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DES_0_A> {
        match self.bits {
            11 => Some(DES_0_A::ARM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ARM`"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == DES_0_A::ARM
    }
}
impl R {
    #[doc = "Bits 0:3 - Bits\\[11:8\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](index.html) module"]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr1::R](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR1 to value 0xbd"]
impl crate::Resettable for PIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbd
    }
}
