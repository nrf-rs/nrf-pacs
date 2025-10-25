#[doc = "Register `READYNEXT` reader"]
pub type R = crate::R<ReadynextSpec>;
#[doc = "NVMC can accept a new write operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readynext {
    #[doc = "0: NVMC cannot accept any write operation"]
    Busy = 0,
    #[doc = "1: NVMC is ready"]
    Ready = 1,
}
impl From<Readynext> for bool {
    #[inline(always)]
    fn from(variant: Readynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - NVMC can accept a new write operation"]
pub type ReadynextR = crate::BitReader<Readynext>;
impl ReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readynext {
        match self.bits {
            false => Readynext::Busy,
            true => Readynext::Ready,
        }
    }
    #[doc = "NVMC cannot accept any write operation"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Readynext::Busy
    }
    #[doc = "NVMC is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Readynext::Ready
    }
}
impl R {
    #[doc = "Bit 0 - NVMC can accept a new write operation"]
    #[inline(always)]
    pub fn readynext(&self) -> ReadynextR {
        ReadynextR::new((self.bits & 1) != 0)
    }
}
#[doc = "Ready flag\n\nYou can [`read`](crate::Reg::read) this register and get [`readynext::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadynextSpec;
impl crate::RegisterSpec for ReadynextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readynext::R`](R) reader structure"]
impl crate::Readable for ReadynextSpec {}
#[doc = "`reset()` method sets READYNEXT to value 0x01"]
impl crate::Resettable for ReadynextSpec {
    const RESET_VALUE: u32 = 0x01;
}
