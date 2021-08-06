#[doc = "Register `RXSTATUS` reader"]
pub struct R(crate::R<RXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status of data in register RXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTATUS_A {
    #[doc = "0: No data pending in register RXDATA"]
    NODATAPENDING = 0,
    #[doc = "1: Data pending in register RXDATA"]
    DATAPENDING = 1,
}
impl From<RXSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTATUS` reader - Status of data in register RXDATA"]
pub struct RXSTATUS_R(crate::FieldReader<bool, RXSTATUS_A>);
impl RXSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTATUS_A {
        match self.bits {
            false => RXSTATUS_A::NODATAPENDING,
            true => RXSTATUS_A::DATAPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NODATAPENDING`"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        **self == RXSTATUS_A::NODATAPENDING
    }
    #[doc = "Checks if the value of the field is `DATAPENDING`"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        **self == RXSTATUS_A::DATAPENDING
    }
}
impl core::ops::Deref for RXSTATUS_R {
    type Target = crate::FieldReader<bool, RXSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register RXDATA"]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "This register shows a status that indicates if data sent from the debugger to the CPU has been read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](index.html) module"]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxstatus::R](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
