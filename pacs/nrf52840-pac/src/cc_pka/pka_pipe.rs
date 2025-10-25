#[doc = "Register `PKA_PIPE` reader"]
pub type R = crate::R<PkaPipeSpec>;
#[doc = "PKA pipeline status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: PKA pipeline is not ready for a new OPCODE"]
    NotReady = 0,
    #[doc = "1: PKA pipeline is ready for a new OPCODE"]
    Ready = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - PKA pipeline status."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::NotReady,
            true => Status::Ready,
        }
    }
    #[doc = "PKA pipeline is not ready for a new OPCODE"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Status::NotReady
    }
    #[doc = "PKA pipeline is ready for a new OPCODE"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Status::Ready
    }
}
impl R {
    #[doc = "Bit 0 - PKA pipeline status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register indicating if the PKA pipeline is ready to receive a new OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_pipe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaPipeSpec;
impl crate::RegisterSpec for PkaPipeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_pipe::R`](R) reader structure"]
impl crate::Readable for PkaPipeSpec {}
#[doc = "`reset()` method sets PKA_PIPE to value 0x01"]
impl crate::Resettable for PkaPipeSpec {
    const RESET_VALUE: u32 = 0x01;
}
