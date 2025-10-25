#[doc = "Register `DIN_CPU_DATA` writer"]
pub type W = crate::W<DinCpuDataSpec>;
#[doc = "Field `SIZE` writer - When using CPU direct write to the DIN_BUFFER, the size of input data in bytes should be written to this register."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - When using CPU direct write to the DIN_BUFFER, the size of input data in bytes should be written to this register."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, DinCpuDataSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Specifies the number of bytes the CPU will write to the DIN_BUFFER, ensuring the cryptographic engine processes the correct amount of data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_cpu_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinCpuDataSpec;
impl crate::RegisterSpec for DinCpuDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`din_cpu_data::W`](W) writer structure"]
impl crate::Writable for DinCpuDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIN_CPU_DATA to value 0"]
impl crate::Resettable for DinCpuDataSpec {}
