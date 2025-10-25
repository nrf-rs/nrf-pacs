#[doc = "Register `MAXCNT` reader"]
pub type R = crate::R<MaxcntSpec>;
#[doc = "Register `MAXCNT` writer"]
pub type W = crate::W<MaxcntSpec>;
#[doc = "Field `MAXCNT` reader - Size of RXD and TXD buffers in number of 32 bit words"]
pub type MaxcntR = crate::FieldReader<u16>;
#[doc = "Field `MAXCNT` writer - Size of RXD and TXD buffers in number of 32 bit words"]
pub type MaxcntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Size of RXD and TXD buffers in number of 32 bit words"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MaxcntR {
        MaxcntR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Size of RXD and TXD buffers in number of 32 bit words"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MaxcntW<'_, MaxcntSpec> {
        MaxcntW::new(self, 0)
    }
}
#[doc = "Size of RXD and TXD buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxcntSpec;
impl crate::RegisterSpec for MaxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxcnt::R`](R) reader structure"]
impl crate::Readable for MaxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`maxcnt::W`](W) writer structure"]
impl crate::Writable for MaxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXCNT to value 0"]
impl crate::Resettable for MaxcntSpec {}
