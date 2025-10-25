#[doc = "Register `DIN_BUFFER` writer"]
pub type W = crate::W<DinBufferSpec>;
#[doc = "Field `DATA` writer - This register is mapped into 8 addresses in order to enable a CPU burst."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - This register is mapped into 8 addresses in order to enable a CPU burst."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DinBufferSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Used by CPU to write data directly to the DIN buffer, which is then sent to the cryptographic engines for processing.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_buffer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinBufferSpec;
impl crate::RegisterSpec for DinBufferSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din_buffer::W`](W) writer structure"]
impl crate::Writable for DinBufferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIN_BUFFER to value 0"]
impl crate::Resettable for DinBufferSpec {}
