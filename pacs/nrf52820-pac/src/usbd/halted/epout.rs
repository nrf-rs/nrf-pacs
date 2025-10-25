#[doc = "Register `EPOUT[%s]` reader"]
pub type R = crate::R<EpoutSpec>;
#[doc = "OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Getstatus {
    #[doc = "0: Endpoint is not halted"]
    NotHalted = 0,
    #[doc = "1: Endpoint is halted"]
    Halted = 1,
}
impl From<Getstatus> for u16 {
    #[inline(always)]
    fn from(variant: Getstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Getstatus {
    type Ux = u16;
}
impl crate::IsEnum for Getstatus {}
#[doc = "Field `GETSTATUS` reader - OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub type GetstatusR = crate::FieldReader<Getstatus>;
impl GetstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Getstatus> {
        match self.bits {
            0 => Some(Getstatus::NotHalted),
            1 => Some(Getstatus::Halted),
            _ => None,
        }
    }
    #[doc = "Endpoint is not halted"]
    #[inline(always)]
    pub fn is_not_halted(&self) -> bool {
        *self == Getstatus::NotHalted
    }
    #[doc = "Endpoint is halted"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == Getstatus::Halted
    }
}
impl R {
    #[doc = "Bits 0:15 - OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn getstatus(&self) -> GetstatusR {
        GetstatusR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Description collection: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`epout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpoutSpec;
impl crate::RegisterSpec for EpoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epout::R`](R) reader structure"]
impl crate::Readable for EpoutSpec {}
#[doc = "`reset()` method sets EPOUT[%s] to value 0"]
impl crate::Resettable for EpoutSpec {}
