#[doc = "Register `BREQUEST` reader"]
pub struct R(crate::R<BREQUEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREQUEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BREQUEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BREQUEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BREQUEST_A {
    #[doc = "0: Standard request GET_STATUS"]
    STD_GET_STATUS = 0,
    #[doc = "1: Standard request CLEAR_FEATURE"]
    STD_CLEAR_FEATURE = 1,
    #[doc = "3: Standard request SET_FEATURE"]
    STD_SET_FEATURE = 3,
    #[doc = "5: Standard request SET_ADDRESS"]
    STD_SET_ADDRESS = 5,
    #[doc = "6: Standard request GET_DESCRIPTOR"]
    STD_GET_DESCRIPTOR = 6,
    #[doc = "7: Standard request SET_DESCRIPTOR"]
    STD_SET_DESCRIPTOR = 7,
    #[doc = "8: Standard request GET_CONFIGURATION"]
    STD_GET_CONFIGURATION = 8,
    #[doc = "9: Standard request SET_CONFIGURATION"]
    STD_SET_CONFIGURATION = 9,
    #[doc = "10: Standard request GET_INTERFACE"]
    STD_GET_INTERFACE = 10,
    #[doc = "11: Standard request SET_INTERFACE"]
    STD_SET_INTERFACE = 11,
    #[doc = "12: Standard request SYNCH_FRAME"]
    STD_SYNCH_FRAME = 12,
}
impl From<BREQUEST_A> for u8 {
    #[inline(always)]
    fn from(variant: BREQUEST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BREQUEST` reader - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
pub struct BREQUEST_R(crate::FieldReader<u8, BREQUEST_A>);
impl BREQUEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BREQUEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BREQUEST_A> {
        match self.bits {
            0 => Some(BREQUEST_A::STD_GET_STATUS),
            1 => Some(BREQUEST_A::STD_CLEAR_FEATURE),
            3 => Some(BREQUEST_A::STD_SET_FEATURE),
            5 => Some(BREQUEST_A::STD_SET_ADDRESS),
            6 => Some(BREQUEST_A::STD_GET_DESCRIPTOR),
            7 => Some(BREQUEST_A::STD_SET_DESCRIPTOR),
            8 => Some(BREQUEST_A::STD_GET_CONFIGURATION),
            9 => Some(BREQUEST_A::STD_SET_CONFIGURATION),
            10 => Some(BREQUEST_A::STD_GET_INTERFACE),
            11 => Some(BREQUEST_A::STD_SET_INTERFACE),
            12 => Some(BREQUEST_A::STD_SYNCH_FRAME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STD_GET_STATUS`"]
    #[inline(always)]
    pub fn is_std_get_status(&self) -> bool {
        **self == BREQUEST_A::STD_GET_STATUS
    }
    #[doc = "Checks if the value of the field is `STD_CLEAR_FEATURE`"]
    #[inline(always)]
    pub fn is_std_clear_feature(&self) -> bool {
        **self == BREQUEST_A::STD_CLEAR_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_FEATURE`"]
    #[inline(always)]
    pub fn is_std_set_feature(&self) -> bool {
        **self == BREQUEST_A::STD_SET_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_ADDRESS`"]
    #[inline(always)]
    pub fn is_std_set_address(&self) -> bool {
        **self == BREQUEST_A::STD_SET_ADDRESS
    }
    #[doc = "Checks if the value of the field is `STD_GET_DESCRIPTOR`"]
    #[inline(always)]
    pub fn is_std_get_descriptor(&self) -> bool {
        **self == BREQUEST_A::STD_GET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_SET_DESCRIPTOR`"]
    #[inline(always)]
    pub fn is_std_set_descriptor(&self) -> bool {
        **self == BREQUEST_A::STD_SET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_GET_CONFIGURATION`"]
    #[inline(always)]
    pub fn is_std_get_configuration(&self) -> bool {
        **self == BREQUEST_A::STD_GET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_SET_CONFIGURATION`"]
    #[inline(always)]
    pub fn is_std_set_configuration(&self) -> bool {
        **self == BREQUEST_A::STD_SET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_GET_INTERFACE`"]
    #[inline(always)]
    pub fn is_std_get_interface(&self) -> bool {
        **self == BREQUEST_A::STD_GET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SET_INTERFACE`"]
    #[inline(always)]
    pub fn is_std_set_interface(&self) -> bool {
        **self == BREQUEST_A::STD_SET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SYNCH_FRAME`"]
    #[inline(always)]
    pub fn is_std_synch_frame(&self) -> bool {
        **self == BREQUEST_A::STD_SYNCH_FRAME
    }
}
impl core::ops::Deref for BREQUEST_R {
    type Target = crate::FieldReader<u8, BREQUEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 1, bRequest\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brequest](index.html) module"]
pub struct BREQUEST_SPEC;
impl crate::RegisterSpec for BREQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brequest::R](R) reader structure"]
impl crate::Readable for BREQUEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BREQUEST to value 0"]
impl crate::Resettable for BREQUEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
