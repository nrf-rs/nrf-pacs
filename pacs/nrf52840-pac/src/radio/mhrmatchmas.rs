#[doc = "Register `MHRMATCHMAS` reader"]
pub type R = crate::R<MhrmatchmasSpec>;
#[doc = "Register `MHRMATCHMAS` writer"]
pub type W = crate::W<MhrmatchmasSpec>;
#[doc = "Field `MHRMATCHMAS` reader - Pattern mask"]
pub type MhrmatchmasR = crate::FieldReader<u32>;
#[doc = "Field `MHRMATCHMAS` writer - Pattern mask"]
pub type MhrmatchmasW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmas(&self) -> MhrmatchmasR {
        MhrmatchmasR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmas(&mut self) -> MhrmatchmasW<'_, MhrmatchmasSpec> {
        MhrmatchmasW::new(self, 0)
    }
}
#[doc = "Pattern mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchmas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchmas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MhrmatchmasSpec;
impl crate::RegisterSpec for MhrmatchmasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mhrmatchmas::R`](R) reader structure"]
impl crate::Readable for MhrmatchmasSpec {}
#[doc = "`write(|w| ..)` method takes [`mhrmatchmas::W`](W) writer structure"]
impl crate::Writable for MhrmatchmasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MHRMATCHMAS to value 0"]
impl crate::Resettable for MhrmatchmasSpec {}
