#[doc = "Register `PDUSTAT` reader"]
pub type R = crate::R<PdustatSpec>;
#[doc = "Status on payload length vs. PCNF1.MAXLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdustat {
    #[doc = "0: Payload less than PCNF1.MAXLEN"]
    LessThan = 0,
    #[doc = "1: Payload greater than PCNF1.MAXLEN"]
    GreaterThan = 1,
}
impl From<Pdustat> for bool {
    #[inline(always)]
    fn from(variant: Pdustat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDUSTAT` reader - Status on payload length vs. PCNF1.MAXLEN"]
pub type PdustatR = crate::BitReader<Pdustat>;
impl PdustatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdustat {
        match self.bits {
            false => Pdustat::LessThan,
            true => Pdustat::GreaterThan,
        }
    }
    #[doc = "Payload less than PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn is_less_than(&self) -> bool {
        *self == Pdustat::LessThan
    }
    #[doc = "Payload greater than PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn is_greater_than(&self) -> bool {
        *self == Pdustat::GreaterThan
    }
}
impl R {
    #[doc = "Bit 0 - Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn pdustat(&self) -> PdustatR {
        PdustatR::new((self.bits & 1) != 0)
    }
}
#[doc = "Payload status\n\nYou can [`read`](crate::Reg::read) this register and get [`pdustat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdustatSpec;
impl crate::RegisterSpec for PdustatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdustat::R`](R) reader structure"]
impl crate::Readable for PdustatSpec {}
#[doc = "`reset()` method sets PDUSTAT to value 0"]
impl crate::Resettable for PdustatSpec {}
