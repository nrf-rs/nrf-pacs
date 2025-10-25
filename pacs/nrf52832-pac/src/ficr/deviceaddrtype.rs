#[doc = "Register `DEVICEADDRTYPE` reader"]
pub type R = crate::R<DeviceaddrtypeSpec>;
#[doc = "Device address type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deviceaddrtype {
    #[doc = "0: Public address"]
    Public = 0,
    #[doc = "1: Random address"]
    Random = 1,
}
impl From<Deviceaddrtype> for bool {
    #[inline(always)]
    fn from(variant: Deviceaddrtype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVICEADDRTYPE` reader - Device address type"]
pub type DeviceaddrtypeR = crate::BitReader<Deviceaddrtype>;
impl DeviceaddrtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Deviceaddrtype {
        match self.bits {
            false => Deviceaddrtype::Public,
            true => Deviceaddrtype::Random,
        }
    }
    #[doc = "Public address"]
    #[inline(always)]
    pub fn is_public(&self) -> bool {
        *self == Deviceaddrtype::Public
    }
    #[doc = "Random address"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        *self == Deviceaddrtype::Random
    }
}
impl R {
    #[doc = "Bit 0 - Device address type"]
    #[inline(always)]
    pub fn deviceaddrtype(&self) -> DeviceaddrtypeR {
        DeviceaddrtypeR::new((self.bits & 1) != 0)
    }
}
#[doc = "Device address type\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddrtype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceaddrtypeSpec;
impl crate::RegisterSpec for DeviceaddrtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceaddrtype::R`](R) reader structure"]
impl crate::Readable for DeviceaddrtypeSpec {}
#[doc = "`reset()` method sets DEVICEADDRTYPE to value 0xffff_ffff"]
impl crate::Resettable for DeviceaddrtypeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
