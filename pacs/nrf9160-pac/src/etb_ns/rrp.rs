#[doc = "Register `RRP` reader"]
pub type R = crate::R<RrpSpec>;
#[doc = "Register `RRP` writer"]
pub type W = crate::W<RrpSpec>;
#[doc = "Field `RAM_READ_POINTER` reader - Sets the read pointer used to read entries from the Trace RAM over the APB interface."]
pub type RamReadPointerR = crate::FieldReader<u16>;
#[doc = "Field `RAM_READ_POINTER` writer - Sets the read pointer used to read entries from the Trace RAM over the APB interface."]
pub type RamReadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sets the read pointer used to read entries from the Trace RAM over the APB interface."]
    #[inline(always)]
    pub fn ram_read_pointer(&self) -> RamReadPointerR {
        RamReadPointerR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the read pointer used to read entries from the Trace RAM over the APB interface."]
    #[inline(always)]
    pub fn ram_read_pointer(&mut self) -> RamReadPointerW<'_, RrpSpec> {
        RamReadPointerW::new(self, 0)
    }
}
#[doc = "ETB RAM Read Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrpSpec;
impl crate::RegisterSpec for RrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrp::R`](R) reader structure"]
impl crate::Readable for RrpSpec {}
#[doc = "`write(|w| ..)` method takes [`rrp::W`](W) writer structure"]
impl crate::Writable for RrpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRP to value 0"]
impl crate::Resettable for RrpSpec {}
