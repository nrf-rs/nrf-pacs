#[doc = "Register `RWP` reader"]
pub type R = crate::R<RwpSpec>;
#[doc = "Register `RWP` writer"]
pub type W = crate::W<RwpSpec>;
#[doc = "Field `RAM_WRITE_POINTER` reader - Sets the write pointer used to write entries from the CoreSight bus into the Trace RAM."]
pub type RamWritePointerR = crate::FieldReader<u16>;
#[doc = "Field `RAM_WRITE_POINTER` writer - Sets the write pointer used to write entries from the CoreSight bus into the Trace RAM."]
pub type RamWritePointerW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sets the write pointer used to write entries from the CoreSight bus into the Trace RAM."]
    #[inline(always)]
    pub fn ram_write_pointer(&self) -> RamWritePointerR {
        RamWritePointerR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the write pointer used to write entries from the CoreSight bus into the Trace RAM."]
    #[inline(always)]
    pub fn ram_write_pointer(&mut self) -> RamWritePointerW<'_, RwpSpec> {
        RamWritePointerW::new(self, 0)
    }
}
#[doc = "ETB RAM Write Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RwpSpec;
impl crate::RegisterSpec for RwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwp::R`](R) reader structure"]
impl crate::Readable for RwpSpec {}
#[doc = "`write(|w| ..)` method takes [`rwp::W`](W) writer structure"]
impl crate::Writable for RwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RWP to value 0"]
impl crate::Resettable for RwpSpec {}
