#[doc = "Register `ORC` reader"]
pub type R = crate::R<OrcSpec>;
#[doc = "Register `ORC` writer"]
pub type W = crate::W<OrcSpec>;
#[doc = "Field `ORC` reader - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub type OrcR = crate::FieldReader;
#[doc = "Field `ORC` writer - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub type OrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(&self) -> OrcR {
        OrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(&mut self) -> OrcW<'_, OrcSpec> {
        OrcW::new(self, 0)
    }
}
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`orc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`orc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrcSpec;
impl crate::RegisterSpec for OrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orc::R`](R) reader structure"]
impl crate::Readable for OrcSpec {}
#[doc = "`write(|w| ..)` method takes [`orc::W`](W) writer structure"]
impl crate::Writable for OrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ORC to value 0"]
impl crate::Resettable for OrcSpec {}
