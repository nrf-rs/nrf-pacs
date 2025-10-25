#[doc = "Register `VALUE[%s]` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE[%s]` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `VALUE` reader - Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ValueSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot.\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE[%s] to value 0xffff_ffff"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
