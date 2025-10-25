#[doc = "Register `ERASEPCR0` writer"]
pub type W = crate::W<Erasepcr0Spec>;
#[doc = "Field `ERASEPCR0` writer - Register for starting erase of a page in code area, equivalent to ERASEPAGE"]
pub type Erasepcr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr0(&mut self) -> Erasepcr0W<'_, Erasepcr0Spec> {
        Erasepcr0W::new(self, 0)
    }
}
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Erasepcr0Spec;
impl crate::RegisterSpec for Erasepcr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`erasepcr0::W`](W) writer structure"]
impl crate::Writable for Erasepcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPCR0 to value 0"]
impl crate::Resettable for Erasepcr0Spec {}
