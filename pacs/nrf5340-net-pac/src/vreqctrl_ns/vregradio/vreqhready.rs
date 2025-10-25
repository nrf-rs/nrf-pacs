#[doc = "Register `VREQHREADY` reader"]
pub type R = crate::R<VreqhreadySpec>;
#[doc = "RADIO is ready to operate on high voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Not ready"]
    NotReady = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - RADIO is ready to operate on high voltage"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::NotReady,
            true => Ready::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Ready::NotReady
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
}
impl R {
    #[doc = "Bit 0 - RADIO is ready to operate on high voltage"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "High voltage on RADIO is ready\n\nYou can [`read`](crate::Reg::read) this register and get [`vreqhready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VreqhreadySpec;
impl crate::RegisterSpec for VreqhreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreqhready::R`](R) reader structure"]
impl crate::Readable for VreqhreadySpec {}
#[doc = "`reset()` method sets VREQHREADY to value 0"]
impl crate::Resettable for VreqhreadySpec {}
