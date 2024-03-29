#[doc = "Register `PDUSTAT` reader"]
pub struct R(crate::R<PDUSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDUSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDUSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDUSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDUSTAT` reader - Status on payload length vs. PCNF1.MAXLEN"]
pub type PDUSTAT_R = crate::BitReader<PDUSTAT_A>;
#[doc = "Status on payload length vs. PCNF1.MAXLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDUSTAT_A {
    #[doc = "0: Payload less than PCNF1.MAXLEN"]
    LESS_THAN = 0,
    #[doc = "1: Payload greater than PCNF1.MAXLEN"]
    GREATER_THAN = 1,
}
impl From<PDUSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PDUSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl PDUSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDUSTAT_A {
        match self.bits {
            false => PDUSTAT_A::LESS_THAN,
            true => PDUSTAT_A::GREATER_THAN,
        }
    }
    #[doc = "Checks if the value of the field is `LESS_THAN`"]
    #[inline(always)]
    pub fn is_less_than(&self) -> bool {
        *self == PDUSTAT_A::LESS_THAN
    }
    #[doc = "Checks if the value of the field is `GREATER_THAN`"]
    #[inline(always)]
    pub fn is_greater_than(&self) -> bool {
        *self == PDUSTAT_A::GREATER_THAN
    }
}
#[doc = "Field `CISTAT` reader - Status on what rate packet is received with in Long Range"]
pub type CISTAT_R = crate::FieldReader<u8, CISTAT_A>;
#[doc = "Status on what rate packet is received with in Long Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CISTAT_A {
    #[doc = "0: Frame is received at 125kbps"]
    LR125KBIT = 0,
    #[doc = "1: Frame is received at 500kbps"]
    LR500KBIT = 1,
}
impl From<CISTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CISTAT_A) -> Self {
        variant as _
    }
}
impl CISTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CISTAT_A> {
        match self.bits {
            0 => Some(CISTAT_A::LR125KBIT),
            1 => Some(CISTAT_A::LR500KBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LR125KBIT`"]
    #[inline(always)]
    pub fn is_lr125kbit(&self) -> bool {
        *self == CISTAT_A::LR125KBIT
    }
    #[doc = "Checks if the value of the field is `LR500KBIT`"]
    #[inline(always)]
    pub fn is_lr500kbit(&self) -> bool {
        *self == CISTAT_A::LR500KBIT
    }
}
impl R {
    #[doc = "Bit 0 - Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn pdustat(&self) -> PDUSTAT_R {
        PDUSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub fn cistat(&self) -> CISTAT_R {
        CISTAT_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "Payload status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdustat](index.html) module"]
pub struct PDUSTAT_SPEC;
impl crate::RegisterSpec for PDUSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdustat::R](R) reader structure"]
impl crate::Readable for PDUSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDUSTAT to value 0"]
impl crate::Resettable for PDUSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
