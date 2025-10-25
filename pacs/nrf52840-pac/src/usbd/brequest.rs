#[doc = "Register `BREQUEST` reader"]
pub type R = crate::R<BrequestSpec>;
#[doc = "SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Brequest {
    #[doc = "0: Standard request GET_STATUS"]
    StdGetStatus = 0,
    #[doc = "1: Standard request CLEAR_FEATURE"]
    StdClearFeature = 1,
    #[doc = "3: Standard request SET_FEATURE"]
    StdSetFeature = 3,
    #[doc = "5: Standard request SET_ADDRESS"]
    StdSetAddress = 5,
    #[doc = "6: Standard request GET_DESCRIPTOR"]
    StdGetDescriptor = 6,
    #[doc = "7: Standard request SET_DESCRIPTOR"]
    StdSetDescriptor = 7,
    #[doc = "8: Standard request GET_CONFIGURATION"]
    StdGetConfiguration = 8,
    #[doc = "9: Standard request SET_CONFIGURATION"]
    StdSetConfiguration = 9,
    #[doc = "10: Standard request GET_INTERFACE"]
    StdGetInterface = 10,
    #[doc = "11: Standard request SET_INTERFACE"]
    StdSetInterface = 11,
    #[doc = "12: Standard request SYNCH_FRAME"]
    StdSynchFrame = 12,
}
impl From<Brequest> for u8 {
    #[inline(always)]
    fn from(variant: Brequest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Brequest {
    type Ux = u8;
}
impl crate::IsEnum for Brequest {}
#[doc = "Field `BREQUEST` reader - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
pub type BrequestR = crate::FieldReader<Brequest>;
impl BrequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Brequest> {
        match self.bits {
            0 => Some(Brequest::StdGetStatus),
            1 => Some(Brequest::StdClearFeature),
            3 => Some(Brequest::StdSetFeature),
            5 => Some(Brequest::StdSetAddress),
            6 => Some(Brequest::StdGetDescriptor),
            7 => Some(Brequest::StdSetDescriptor),
            8 => Some(Brequest::StdGetConfiguration),
            9 => Some(Brequest::StdSetConfiguration),
            10 => Some(Brequest::StdGetInterface),
            11 => Some(Brequest::StdSetInterface),
            12 => Some(Brequest::StdSynchFrame),
            _ => None,
        }
    }
    #[doc = "Standard request GET_STATUS"]
    #[inline(always)]
    pub fn is_std_get_status(&self) -> bool {
        *self == Brequest::StdGetStatus
    }
    #[doc = "Standard request CLEAR_FEATURE"]
    #[inline(always)]
    pub fn is_std_clear_feature(&self) -> bool {
        *self == Brequest::StdClearFeature
    }
    #[doc = "Standard request SET_FEATURE"]
    #[inline(always)]
    pub fn is_std_set_feature(&self) -> bool {
        *self == Brequest::StdSetFeature
    }
    #[doc = "Standard request SET_ADDRESS"]
    #[inline(always)]
    pub fn is_std_set_address(&self) -> bool {
        *self == Brequest::StdSetAddress
    }
    #[doc = "Standard request GET_DESCRIPTOR"]
    #[inline(always)]
    pub fn is_std_get_descriptor(&self) -> bool {
        *self == Brequest::StdGetDescriptor
    }
    #[doc = "Standard request SET_DESCRIPTOR"]
    #[inline(always)]
    pub fn is_std_set_descriptor(&self) -> bool {
        *self == Brequest::StdSetDescriptor
    }
    #[doc = "Standard request GET_CONFIGURATION"]
    #[inline(always)]
    pub fn is_std_get_configuration(&self) -> bool {
        *self == Brequest::StdGetConfiguration
    }
    #[doc = "Standard request SET_CONFIGURATION"]
    #[inline(always)]
    pub fn is_std_set_configuration(&self) -> bool {
        *self == Brequest::StdSetConfiguration
    }
    #[doc = "Standard request GET_INTERFACE"]
    #[inline(always)]
    pub fn is_std_get_interface(&self) -> bool {
        *self == Brequest::StdGetInterface
    }
    #[doc = "Standard request SET_INTERFACE"]
    #[inline(always)]
    pub fn is_std_set_interface(&self) -> bool {
        *self == Brequest::StdSetInterface
    }
    #[doc = "Standard request SYNCH_FRAME"]
    #[inline(always)]
    pub fn is_std_synch_frame(&self) -> bool {
        *self == Brequest::StdSynchFrame
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline(always)]
    pub fn brequest(&self) -> BrequestR {
        BrequestR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 1, bRequest\n\nYou can [`read`](crate::Reg::read) this register and get [`brequest::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrequestSpec;
impl crate::RegisterSpec for BrequestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brequest::R`](R) reader structure"]
impl crate::Readable for BrequestSpec {}
#[doc = "`reset()` method sets BREQUEST to value 0"]
impl crate::Resettable for BrequestSpec {}
