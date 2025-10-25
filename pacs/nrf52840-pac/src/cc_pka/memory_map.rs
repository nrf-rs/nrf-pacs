#[doc = "Register `MEMORY_MAP[%s]` reader"]
pub type R = crate::R<MemoryMapSpec>;
#[doc = "Register `MEMORY_MAP[%s]` writer"]
pub type W = crate::W<MemoryMapSpec>;
#[doc = "Field `ADDR` reader - The physical word address used for the virtual register."]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - The physical word address used for the virtual register."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 1:9 - The physical word address used for the virtual register."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:9 - The physical word address used for the virtual register."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, MemoryMapSpec> {
        AddrW::new(self, 1)
    }
}
#[doc = "Description collection: Register for mapping the virtual register R\\[n\\] to a physical address in the PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`memory_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memory_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemoryMapSpec;
impl crate::RegisterSpec for MemoryMapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memory_map::R`](R) reader structure"]
impl crate::Readable for MemoryMapSpec {}
#[doc = "`write(|w| ..)` method takes [`memory_map::W`](W) writer structure"]
impl crate::Writable for MemoryMapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMORY_MAP[%s] to value 0"]
impl crate::Resettable for MemoryMapSpec {}
