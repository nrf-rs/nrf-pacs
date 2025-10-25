#[doc = "Register `RDP` reader"]
pub type R = crate::R<RdpSpec>;
#[doc = "Field `ETB_RAM_DEPTH` reader - Defines the depth, in words, of the trace RAM."]
pub type EtbRamDepthR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the depth, in words, of the trace RAM."]
    #[inline(always)]
    pub fn etb_ram_depth(&self) -> EtbRamDepthR {
        EtbRamDepthR::new(self.bits)
    }
}
#[doc = "ETB RAM Depth Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdpSpec;
impl crate::RegisterSpec for RdpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdp::R`](R) reader structure"]
impl crate::Readable for RdpSpec {}
#[doc = "`reset()` method sets RDP to value 0"]
impl crate::Resettable for RdpSpec {}
