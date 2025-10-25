#[doc = "Register `PARTNO` reader"]
pub type R = crate::R<PartnoSpec>;
#[doc = "\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Partno {
    #[doc = "37217: Device is an nRF9161 sip"]
    _9161 = 37217,
    #[doc = "37216: Device is an nRF9160 sip"]
    _9160 = 37216,
    #[doc = "37201: Device is an nRF9151 sip"]
    _9151 = 37201,
    #[doc = "37169: Device is an nRF9131 sip"]
    _9131 = 37169,
}
impl From<Partno> for u32 {
    #[inline(always)]
    fn from(variant: Partno) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Partno {
    type Ux = u32;
}
impl crate::IsEnum for Partno {}
#[doc = "Field `PARTNO` reader - "]
pub type PartnoR = crate::FieldReader<Partno>;
impl PartnoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Partno> {
        match self.bits {
            37217 => Some(Partno::_9161),
            37216 => Some(Partno::_9160),
            37201 => Some(Partno::_9151),
            37169 => Some(Partno::_9131),
            _ => None,
        }
    }
    #[doc = "Device is an nRF9161 sip"]
    #[inline(always)]
    pub fn is_9161(&self) -> bool {
        *self == Partno::_9161
    }
    #[doc = "Device is an nRF9160 sip"]
    #[inline(always)]
    pub fn is_9160(&self) -> bool {
        *self == Partno::_9160
    }
    #[doc = "Device is an nRF9151 sip"]
    #[inline(always)]
    pub fn is_9151(&self) -> bool {
        *self == Partno::_9151
    }
    #[doc = "Device is an nRF9131 sip"]
    #[inline(always)]
    pub fn is_9131(&self) -> bool {
        *self == Partno::_9131
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(self.bits)
    }
}
#[doc = "SIP part number\n\nYou can [`read`](crate::Reg::read) this register and get [`partno::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartnoSpec;
impl crate::RegisterSpec for PartnoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`partno::R`](R) reader structure"]
impl crate::Readable for PartnoSpec {}
#[doc = "`reset()` method sets PARTNO to value 0xffff_ffff"]
impl crate::Resettable for PartnoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
