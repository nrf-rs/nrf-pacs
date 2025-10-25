#[doc = "Register `BASE0` reader"]
pub type R = crate::R<Base0Spec>;
#[doc = "Register `BASE0` writer"]
pub type W = crate::W<Base0Spec>;
#[doc = "Field `BASE0` reader - Base address 0"]
pub type Base0R = crate::FieldReader<u32>;
#[doc = "Field `BASE0` writer - Base address 0"]
pub type Base0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&self) -> Base0R {
        Base0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address 0"]
    #[inline(always)]
    pub fn base0(&mut self) -> Base0W<'_, Base0Spec> {
        Base0W::new(self, 0)
    }
}
#[doc = "Base address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`base0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Base0Spec;
impl crate::RegisterSpec for Base0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base0::R`](R) reader structure"]
impl crate::Readable for Base0Spec {}
#[doc = "`write(|w| ..)` method takes [`base0::W`](W) writer structure"]
impl crate::Writable for Base0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE0 to value 0"]
impl crate::Resettable for Base0Spec {}
