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
#[doc = "Status on what rate packet is received with in Long Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cistat {
    #[doc = "0: Frame is received at 125 kbps"]
    Lr125kbit = 0,
    #[doc = "1: Frame is received at 500 kbps"]
    Lr500kbit = 1,
}
impl From<Cistat> for u8 {
    #[inline(always)]
    fn from(variant: Cistat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cistat {
    type Ux = u8;
}
impl crate::IsEnum for Cistat {}
#[doc = "Field `CISTAT` reader - Status on what rate packet is received with in Long Range"]
pub type CistatR = crate::FieldReader<Cistat>;
impl CistatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cistat> {
        match self.bits {
            0 => Some(Cistat::Lr125kbit),
            1 => Some(Cistat::Lr500kbit),
            _ => None,
        }
    }
    #[doc = "Frame is received at 125 kbps"]
    #[inline(always)]
    pub fn is_lr125kbit(&self) -> bool {
        *self == Cistat::Lr125kbit
    }
    #[doc = "Frame is received at 500 kbps"]
    #[inline(always)]
    pub fn is_lr500kbit(&self) -> bool {
        *self == Cistat::Lr500kbit
    }
}
impl R {
    #[doc = "Bit 0 - Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn pdustat(&self) -> PdustatR {
        PdustatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub fn cistat(&self) -> CistatR {
        CistatR::new(((self.bits >> 1) & 3) as u8)
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
