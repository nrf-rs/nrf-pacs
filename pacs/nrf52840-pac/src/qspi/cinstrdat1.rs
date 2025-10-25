#[doc = "Register `CINSTRDAT1` reader"]
pub type R = crate::R<Cinstrdat1Spec>;
#[doc = "Register `CINSTRDAT1` writer"]
pub type W = crate::W<Cinstrdat1Spec>;
#[doc = "Field `BYTE4` reader - Data byte 4"]
pub type Byte4R = crate::FieldReader;
#[doc = "Field `BYTE4` writer - Data byte 4"]
pub type Byte4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE5` reader - Data byte 5"]
pub type Byte5R = crate::FieldReader;
#[doc = "Field `BYTE5` writer - Data byte 5"]
pub type Byte5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE6` reader - Data byte 6"]
pub type Byte6R = crate::FieldReader;
#[doc = "Field `BYTE6` writer - Data byte 6"]
pub type Byte6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE7` reader - Data byte 7"]
pub type Byte7R = crate::FieldReader;
#[doc = "Field `BYTE7` writer - Data byte 7"]
pub type Byte7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> Byte4R {
        Byte4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> Byte5R {
        Byte5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> Byte6R {
        Byte6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> Byte7R {
        Byte7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> Byte4W<'_, Cinstrdat1Spec> {
        Byte4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> Byte5W<'_, Cinstrdat1Spec> {
        Byte5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> Byte6W<'_, Cinstrdat1Spec> {
        Byte6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> Byte7W<'_, Cinstrdat1Spec> {
        Byte7W::new(self, 24)
    }
}
#[doc = "Custom instruction data register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrdat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrdat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cinstrdat1Spec;
impl crate::RegisterSpec for Cinstrdat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cinstrdat1::R`](R) reader structure"]
impl crate::Readable for Cinstrdat1Spec {}
#[doc = "`write(|w| ..)` method takes [`cinstrdat1::W`](W) writer structure"]
impl crate::Writable for Cinstrdat1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CINSTRDAT1 to value 0"]
impl crate::Resettable for Cinstrdat1Spec {}
