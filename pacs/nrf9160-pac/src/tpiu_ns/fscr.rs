#[doc = "Register `FSCR` reader"]
pub type R = crate::R<FscrSpec>;
#[doc = "Register `FSCR` writer"]
pub type W = crate::W<FscrSpec>;
#[doc = "Field `CYCCOUNT` reader - 12-bit counter reload value. Indicates the number of complete frames between full synchronization packets."]
pub type CyccountR = crate::FieldReader<u16>;
#[doc = "Field `CYCCOUNT` writer - 12-bit counter reload value. Indicates the number of complete frames between full synchronization packets."]
pub type CyccountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 12-bit counter reload value. Indicates the number of complete frames between full synchronization packets."]
    #[inline(always)]
    pub fn cyccount(&self) -> CyccountR {
        CyccountR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 12-bit counter reload value. Indicates the number of complete frames between full synchronization packets."]
    #[inline(always)]
    pub fn cyccount(&mut self) -> CyccountW<'_, FscrSpec> {
        CyccountW::new(self, 0)
    }
}
#[doc = "The FSCR register enables the frequency of synchronization information to be optimized to suit the Trace Port Analyzer (TPA) capture buffer size.\n\nYou can [`read`](crate::Reg::read) this register and get [`fscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FscrSpec;
impl crate::RegisterSpec for FscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscr::R`](R) reader structure"]
impl crate::Readable for FscrSpec {}
#[doc = "`write(|w| ..)` method takes [`fscr::W`](W) writer structure"]
impl crate::Writable for FscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSCR to value 0"]
impl crate::Resettable for FscrSpec {}
