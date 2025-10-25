#[doc = "Register `ISOOUT` reader"]
pub type R = crate::R<IsooutSpec>;
#[doc = "Field `SIZE` reader - Number of bytes received last on this ISO OUT data endpoint"]
pub type SizeR = crate::FieldReader<u16>;
#[doc = "Zero-length data packet received\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zero {
    #[doc = "0: No zero-length data received, use value in SIZE"]
    Normal = 0,
    #[doc = "1: Zero-length data received, ignore value in SIZE"]
    ZeroData = 1,
}
impl From<Zero> for bool {
    #[inline(always)]
    fn from(variant: Zero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZERO` reader - Zero-length data packet received"]
pub type ZeroR = crate::BitReader<Zero>;
impl ZeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zero {
        match self.bits {
            false => Zero::Normal,
            true => Zero::ZeroData,
        }
    }
    #[doc = "No zero-length data received, use value in SIZE"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Zero::Normal
    }
    #[doc = "Zero-length data received, ignore value in SIZE"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == Zero::ZeroData
    }
}
impl R {
    #[doc = "Bits 0:9 - Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Number of bytes received last on this ISO OUT data endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`isoout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsooutSpec;
impl crate::RegisterSpec for IsooutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoout::R`](R) reader structure"]
impl crate::Readable for IsooutSpec {}
#[doc = "`reset()` method sets ISOOUT to value 0x0001_0000"]
impl crate::Resettable for IsooutSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
