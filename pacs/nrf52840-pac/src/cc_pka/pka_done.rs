#[doc = "Register `PKA_DONE` reader"]
pub type R = crate::R<PkaDoneSpec>;
#[doc = "PKA operation status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: PKA operation is processing"]
    Processing = 0,
    #[doc = "1: PKA operation is completed and pipeline is empty"]
    Completed = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - PKA operation status."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Processing,
            true => Status::Completed,
        }
    }
    #[doc = "PKA operation is processing"]
    #[inline(always)]
    pub fn is_processing(&self) -> bool {
        *self == Status::Processing
    }
    #[doc = "PKA operation is completed and pipeline is empty"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Status::Completed
    }
}
impl R {
    #[doc = "Bit 0 - PKA operation status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register indicating if the PKA operation has been completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaDoneSpec;
impl crate::RegisterSpec for PkaDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_done::R`](R) reader structure"]
impl crate::Readable for PkaDoneSpec {}
#[doc = "`reset()` method sets PKA_DONE to value 0x01"]
impl crate::Resettable for PkaDoneSpec {
    const RESET_VALUE: u32 = 0x01;
}
