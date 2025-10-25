#[doc = "Register `MAXLEN` reader"]
pub type R = crate::R<MaxlenSpec>;
#[doc = "Register `MAXLEN` writer"]
pub type W = crate::W<MaxlenSpec>;
#[doc = "Field `MAXLEN` reader - Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub type MaxlenR = crate::FieldReader<u16>;
#[doc = "Field `MAXLEN` writer - Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub type MaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn maxlen(&self) -> MaxlenR {
        MaxlenR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MaxlenW<'_, MaxlenSpec> {
        MaxlenW::new(self, 0)
    }
}
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each\n\nYou can [`read`](crate::Reg::read) this register and get [`maxlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxlenSpec;
impl crate::RegisterSpec for MaxlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxlen::R`](R) reader structure"]
impl crate::Readable for MaxlenSpec {}
#[doc = "`write(|w| ..)` method takes [`maxlen::W`](W) writer structure"]
impl crate::Writable for MaxlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXLEN to value 0"]
impl crate::Resettable for MaxlenSpec {}
