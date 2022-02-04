#[doc = "Register `MAINREGSTATUS` reader"]
pub struct R(crate::R<MAINREGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINREGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINREGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINREGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Main supply status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINREGSTATUS_A {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD."]
    NORMAL = 0,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    HIGH = 1,
}
impl From<MAINREGSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MAINREGSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAINREGSTATUS` reader - Main supply status"]
pub struct MAINREGSTATUS_R(crate::FieldReader<bool, MAINREGSTATUS_A>);
impl MAINREGSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAINREGSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAINREGSTATUS_A {
        match self.bits {
            false => MAINREGSTATUS_A::NORMAL,
            true => MAINREGSTATUS_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MAINREGSTATUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == MAINREGSTATUS_A::HIGH
    }
}
impl core::ops::Deref for MAINREGSTATUS_R {
    type Target = crate::FieldReader<bool, MAINREGSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Main supply status"]
    #[inline(always)]
    pub fn mainregstatus(&self) -> MAINREGSTATUS_R {
        MAINREGSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Main supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainregstatus](index.html) module"]
pub struct MAINREGSTATUS_SPEC;
impl crate::RegisterSpec for MAINREGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainregstatus::R](R) reader structure"]
impl crate::Readable for MAINREGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAINREGSTATUS to value 0"]
impl crate::Resettable for MAINREGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
