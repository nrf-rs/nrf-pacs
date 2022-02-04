#[doc = "Register `POWERSTATUS` reader"]
pub struct R(crate::R<POWERSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWERSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWERSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWERSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LTE modem domain status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTEMODEM_A {
    #[doc = "0: LTE modem domain is powered off"]
    OFF = 0,
    #[doc = "1: LTE modem domain is powered on"]
    ON = 1,
}
impl From<LTEMODEM_A> for bool {
    #[inline(always)]
    fn from(variant: LTEMODEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTEMODEM` reader - LTE modem domain status"]
pub struct LTEMODEM_R(crate::FieldReader<bool, LTEMODEM_A>);
impl LTEMODEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LTEMODEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTEMODEM_A {
        match self.bits {
            false => LTEMODEM_A::OFF,
            true => LTEMODEM_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == LTEMODEM_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == LTEMODEM_A::ON
    }
}
impl core::ops::Deref for LTEMODEM_R {
    type Target = crate::FieldReader<bool, LTEMODEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LTE modem domain status"]
    #[inline(always)]
    pub fn ltemodem(&self) -> LTEMODEM_R {
        LTEMODEM_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Modem domain power status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerstatus](index.html) module"]
pub struct POWERSTATUS_SPEC;
impl crate::RegisterSpec for POWERSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerstatus::R](R) reader structure"]
impl crate::Readable for POWERSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POWERSTATUS to value 0"]
impl crate::Resettable for POWERSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
