#[doc = "Register `PIDR0` reader"]
pub struct R(crate::R<PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Bits\\[7:0\\]
of the 12-bit part number of the component. The designer of the component assigns this part number.\n\nValue on reset: 33"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PART_0_A {
    #[doc = "33: Indicates bits\\[7:0\\]
of the part number of the component."]
    PARTNUMBERL = 33,
}
impl From<PART_0_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PART_0` reader - Bits\\[7:0\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
pub struct PART_0_R(crate::FieldReader<u8, PART_0_A>);
impl PART_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PART_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PART_0_A> {
        match self.bits {
            33 => Some(PART_0_A::PARTNUMBERL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PARTNUMBERL`"]
    #[inline(always)]
    pub fn is_partnumber_l(&self) -> bool {
        **self == PART_0_A::PARTNUMBERL
    }
}
impl core::ops::Deref for PART_0_R {
    type Target = crate::FieldReader<u8, PART_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits\\[7:0\\]
of the 12-bit part number of the component. The designer of the component assigns this part number."]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](index.html) module"]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr0::R](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR0 to value 0x21"]
impl crate::Resettable for PIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
