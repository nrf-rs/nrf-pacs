#[doc = "Register `DEVICETYPE` reader"]
pub type R = crate::R<DevicetypeSpec>;
#[doc = "Device type\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Devicetype {
    #[doc = "0: Device is an physical DIE"]
    Die = 0,
    #[doc = "4294967295: Device is an FPGA"]
    Fpga = 4294967295,
}
impl From<Devicetype> for u32 {
    #[inline(always)]
    fn from(variant: Devicetype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devicetype {
    type Ux = u32;
}
impl crate::IsEnum for Devicetype {}
#[doc = "Field `DEVICETYPE` reader - Device type"]
pub type DevicetypeR = crate::FieldReader<Devicetype>;
impl DevicetypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devicetype> {
        match self.bits {
            0 => Some(Devicetype::Die),
            4294967295 => Some(Devicetype::Fpga),
            _ => None,
        }
    }
    #[doc = "Device is an physical DIE"]
    #[inline(always)]
    pub fn is_die(&self) -> bool {
        *self == Devicetype::Die
    }
    #[doc = "Device is an FPGA"]
    #[inline(always)]
    pub fn is_fpga(&self) -> bool {
        *self == Devicetype::Fpga
    }
}
impl R {
    #[doc = "Bits 0:31 - Device type"]
    #[inline(always)]
    pub fn devicetype(&self) -> DevicetypeR {
        DevicetypeR::new(self.bits)
    }
}
#[doc = "Device type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devicetype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicetypeSpec;
impl crate::RegisterSpec for DevicetypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devicetype::R`](R) reader structure"]
impl crate::Readable for DevicetypeSpec {}
#[doc = "`reset()` method sets DEVICETYPE to value 0xffff_ffff"]
impl crate::Resettable for DevicetypeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
