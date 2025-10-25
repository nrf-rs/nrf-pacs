#[doc = "Register `DEF` reader"]
pub type R = crate::R<DefSpec>;
#[doc = "Register `DEF` writer"]
pub type W = crate::W<DefSpec>;
#[doc = "Field `DEF` reader - Default character. Character clocked out in case of an ignored transaction."]
pub type DefR = crate::FieldReader;
#[doc = "Field `DEF` writer - Default character. Character clocked out in case of an ignored transaction."]
pub type DefW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&self) -> DefR {
        DefR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&mut self) -> DefW<'_, DefSpec> {
        DefW::new(self, 0)
    }
}
#[doc = "Default character. Character clocked out in case of an ignored transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`def::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`def::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DefSpec;
impl crate::RegisterSpec for DefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`def::R`](R) reader structure"]
impl crate::Readable for DefSpec {}
#[doc = "`write(|w| ..)` method takes [`def::W`](W) writer structure"]
impl crate::Writable for DefSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEF to value 0"]
impl crate::Resettable for DefSpec {}
