#[doc = "Register `CRYPTO_BUSY` reader"]
pub type R = crate::R<CryptoBusySpec>;
#[doc = "Cryptographic core engines status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Cryptographic core engines are idle"]
    Idle = 0,
    #[doc = "1: Cryptographic core engines are busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Cryptographic core engines status."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Idle,
            true => Status::Busy,
        }
    }
    #[doc = "Cryptographic core engines are idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "Cryptographic core engines are busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Cryptographic core engines status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register for cryptographic cores engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoBusySpec;
impl crate::RegisterSpec for CryptoBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_busy::R`](R) reader structure"]
impl crate::Readable for CryptoBusySpec {}
#[doc = "`reset()` method sets CRYPTO_BUSY to value 0"]
impl crate::Resettable for CryptoBusySpec {}
