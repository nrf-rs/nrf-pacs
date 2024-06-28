#[doc = "Register `ORC` reader"]
pub type R = crate::R<OrcSpec>;
#[doc = "Register `ORC` writer"]
pub type W = crate::W<OrcSpec>;
#[doc = "Field `ORC` reader - Over-read character. Character clocked out after an over-read of the transmit buffer."]
pub type OrcR = crate::FieldReader;
#[doc = "Field `ORC` writer - Over-read character. Character clocked out after an over-read of the transmit buffer."]
pub type OrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Over-read character. Character clocked out after an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(&self) -> OrcR {
        OrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Over-read character. Character clocked out after an over-read of the transmit buffer."]
    #[inline(always)]
    #[must_use]
    pub fn orc(&mut self) -> OrcW<OrcSpec> {
        OrcW::new(self, 0)
    }
}
#[doc = "Over-read character\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`orc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrcSpec;
impl crate::RegisterSpec for OrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`orc::R`](R) reader structure"]
impl crate::Readable for OrcSpec {}
#[doc = "`write(|w| ..)` method takes [`orc::W`](W) writer structure"]
impl crate::Writable for OrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ORC to value 0"]
impl crate::Resettable for OrcSpec {
    const RESET_VALUE: u32 = 0;
}
