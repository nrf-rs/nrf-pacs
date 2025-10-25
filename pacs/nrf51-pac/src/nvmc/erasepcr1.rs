#[doc = "Register `ERASEPCR1` reader"]
pub type R = crate::R<Erasepcr1Spec>;
#[doc = "Register `ERASEPCR1` writer"]
pub type W = crate::W<Erasepcr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Register for erasing a non-protected non-volatile memory page.\n\nYou can [`read`](crate::Reg::read) this register and get [`erasepcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Erasepcr1Spec;
impl crate::RegisterSpec for Erasepcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasepcr1::R`](R) reader structure"]
impl crate::Readable for Erasepcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`erasepcr1::W`](W) writer structure"]
impl crate::Writable for Erasepcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPCR1 to value 0"]
impl crate::Resettable for Erasepcr1Spec {}
