#[doc = "Register `BMREQUESTTYPE` reader"]
pub struct R(crate::R<BMREQUESTTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMREQUESTTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMREQUESTTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMREQUESTTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECIPIENT` reader - Data transfer type"]
pub type RECIPIENT_R = crate::FieldReader<u8, RECIPIENT_A>;
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RECIPIENT_A {
    #[doc = "0: Device"]
    DEVICE = 0,
    #[doc = "1: Interface"]
    INTERFACE = 1,
    #[doc = "2: Endpoint"]
    ENDPOINT = 2,
    #[doc = "3: Other"]
    OTHER = 3,
}
impl From<RECIPIENT_A> for u8 {
    #[inline(always)]
    fn from(variant: RECIPIENT_A) -> Self {
        variant as _
    }
}
impl RECIPIENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECIPIENT_A> {
        match self.bits {
            0 => Some(RECIPIENT_A::DEVICE),
            1 => Some(RECIPIENT_A::INTERFACE),
            2 => Some(RECIPIENT_A::ENDPOINT),
            3 => Some(RECIPIENT_A::OTHER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == RECIPIENT_A::DEVICE
    }
    #[doc = "Checks if the value of the field is `INTERFACE`"]
    #[inline(always)]
    pub fn is_interface(&self) -> bool {
        *self == RECIPIENT_A::INTERFACE
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == RECIPIENT_A::ENDPOINT
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == RECIPIENT_A::OTHER
    }
}
#[doc = "Field `TYPE` reader - Data transfer type"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Standard"]
    STANDARD = 0,
    #[doc = "1: Class"]
    CLASS = 1,
    #[doc = "2: Vendor"]
    VENDOR = 2,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPE_A> {
        match self.bits {
            0 => Some(TYPE_A::STANDARD),
            1 => Some(TYPE_A::CLASS),
            2 => Some(TYPE_A::VENDOR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TYPE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `CLASS`"]
    #[inline(always)]
    pub fn is_class(&self) -> bool {
        *self == TYPE_A::CLASS
    }
    #[doc = "Checks if the value of the field is `VENDOR`"]
    #[inline(always)]
    pub fn is_vendor(&self) -> bool {
        *self == TYPE_A::VENDOR
    }
}
#[doc = "Field `DIRECTION` reader - Data transfer direction"]
pub type DIRECTION_R = crate::BitReader<DIRECTION_A>;
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTION_A {
    #[doc = "0: Host-to-device"]
    HOST_TO_DEVICE = 0,
    #[doc = "1: Device-to-host"]
    DEVICE_TO_HOST = 1,
}
impl From<DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTION_A {
        match self.bits {
            false => DIRECTION_A::HOST_TO_DEVICE,
            true => DIRECTION_A::DEVICE_TO_HOST,
        }
    }
    #[doc = "Checks if the value of the field is `HOST_TO_DEVICE`"]
    #[inline(always)]
    pub fn is_host_to_device(&self) -> bool {
        *self == DIRECTION_A::HOST_TO_DEVICE
    }
    #[doc = "Checks if the value of the field is `DEVICE_TO_HOST`"]
    #[inline(always)]
    pub fn is_device_to_host(&self) -> bool {
        *self == DIRECTION_A::DEVICE_TO_HOST
    }
}
impl R {
    #[doc = "Bits 0:4 - Data transfer type"]
    #[inline(always)]
    pub fn recipient(&self) -> RECIPIENT_R {
        RECIPIENT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Data transfer type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SETUP data, byte 0, bmRequestType\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmrequesttype](index.html) module"]
pub struct BMREQUESTTYPE_SPEC;
impl crate::RegisterSpec for BMREQUESTTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmrequesttype::R](R) reader structure"]
impl crate::Readable for BMREQUESTTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BMREQUESTTYPE to value 0"]
impl crate::Resettable for BMREQUESTTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
