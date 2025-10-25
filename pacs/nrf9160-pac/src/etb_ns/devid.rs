#[doc = "Register `DEVID` reader"]
pub type R = crate::R<DevidSpec>;
#[doc = "Field `EXTMUXNUM` reader - When non-zero this value indicates the type/number of ATB multiplexing present on the input to the ATB."]
pub type ExtmuxnumR = crate::FieldReader;
#[doc = "Field `RAMCLK` reader - This bit returns 0 on reads indicating that the ETB RAM operates synchronously to atclk."]
pub type RamclkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - When non-zero this value indicates the type/number of ATB multiplexing present on the input to the ATB."]
    #[inline(always)]
    pub fn extmuxnum(&self) -> ExtmuxnumR {
        ExtmuxnumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - This bit returns 0 on reads indicating that the ETB RAM operates synchronously to atclk."]
    #[inline(always)]
    pub fn ramclk(&self) -> RamclkR {
        RamclkR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevidSpec;
impl crate::RegisterSpec for DevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devid::R`](R) reader structure"]
impl crate::Readable for DevidSpec {}
#[doc = "`reset()` method sets DEVID to value 0"]
impl crate::Resettable for DevidSpec {}
