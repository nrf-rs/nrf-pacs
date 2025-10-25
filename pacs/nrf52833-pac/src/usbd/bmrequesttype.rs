#[doc = "Register `BMREQUESTTYPE` reader"]
pub type R = crate::R<BmrequesttypeSpec>;
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Recipient {
    #[doc = "0: Device"]
    Device = 0,
    #[doc = "1: Interface"]
    Interface = 1,
    #[doc = "2: Endpoint"]
    Endpoint = 2,
    #[doc = "3: Other"]
    Other = 3,
}
impl From<Recipient> for u8 {
    #[inline(always)]
    fn from(variant: Recipient) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Recipient {
    type Ux = u8;
}
impl crate::IsEnum for Recipient {}
#[doc = "Field `RECIPIENT` reader - Data transfer type"]
pub type RecipientR = crate::FieldReader<Recipient>;
impl RecipientR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Recipient> {
        match self.bits {
            0 => Some(Recipient::Device),
            1 => Some(Recipient::Interface),
            2 => Some(Recipient::Endpoint),
            3 => Some(Recipient::Other),
            _ => None,
        }
    }
    #[doc = "Device"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == Recipient::Device
    }
    #[doc = "Interface"]
    #[inline(always)]
    pub fn is_interface(&self) -> bool {
        *self == Recipient::Interface
    }
    #[doc = "Endpoint"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == Recipient::Endpoint
    }
    #[doc = "Other"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Recipient::Other
    }
}
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: Standard"]
    Standard = 0,
    #[doc = "1: Class"]
    Class = 1,
    #[doc = "2: Vendor"]
    Vendor = 2,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - Data transfer type"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            0 => Some(Type::Standard),
            1 => Some(Type::Class),
            2 => Some(Type::Vendor),
            _ => None,
        }
    }
    #[doc = "Standard"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Type::Standard
    }
    #[doc = "Class"]
    #[inline(always)]
    pub fn is_class(&self) -> bool {
        *self == Type::Class
    }
    #[doc = "Vendor"]
    #[inline(always)]
    pub fn is_vendor(&self) -> bool {
        *self == Type::Vendor
    }
}
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    #[doc = "0: Host-to-device"]
    HostToDevice = 0,
    #[doc = "1: Device-to-host"]
    DeviceToHost = 1,
}
impl From<Direction> for bool {
    #[inline(always)]
    fn from(variant: Direction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECTION` reader - Data transfer direction"]
pub type DirectionR = crate::BitReader<Direction>;
impl DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Direction {
        match self.bits {
            false => Direction::HostToDevice,
            true => Direction::DeviceToHost,
        }
    }
    #[doc = "Host-to-device"]
    #[inline(always)]
    pub fn is_host_to_device(&self) -> bool {
        *self == Direction::HostToDevice
    }
    #[doc = "Device-to-host"]
    #[inline(always)]
    pub fn is_device_to_host(&self) -> bool {
        *self == Direction::DeviceToHost
    }
}
impl R {
    #[doc = "Bits 0:4 - Data transfer type"]
    #[inline(always)]
    pub fn recipient(&self) -> RecipientR {
        RecipientR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Data transfer type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SETUP data, byte 0, bmRequestType\n\nYou can [`read`](crate::Reg::read) this register and get [`bmrequesttype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmrequesttypeSpec;
impl crate::RegisterSpec for BmrequesttypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmrequesttype::R`](R) reader structure"]
impl crate::Readable for BmrequesttypeSpec {}
#[doc = "`reset()` method sets BMREQUESTTYPE to value 0"]
impl crate::Resettable for BmrequesttypeSpec {}
