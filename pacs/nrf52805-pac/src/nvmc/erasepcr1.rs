#[doc = "Register `ERASEPCR1` writer"]
pub type W = crate::W<Erasepcr1Spec>;
#[doc = "Field `ERASEPCR1` writer - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub type Erasepcr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr1(&mut self) -> Erasepcr1W<'_, Erasepcr1Spec> {
        Erasepcr1W::new(self, 0)
    }
}
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Erasepcr1Spec;
impl crate::RegisterSpec for Erasepcr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`erasepcr1::W`](W) writer structure"]
impl crate::Writable for Erasepcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPCR1 to value 0"]
impl crate::Resettable for Erasepcr1Spec {}
