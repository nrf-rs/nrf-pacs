#[doc = "Register `AES_CTR_NO_INCREMENT` reader"]
pub type R = crate::R<AesCtrNoIncrementSpec>;
#[doc = "Register `AES_CTR_NO_INCREMENT` writer"]
pub type W = crate::W<AesCtrNoIncrementSpec>;
#[doc = "This field enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Counter always incremented between blocks"]
    Disable = 0,
    #[doc = "1: Do not increment counter between blocks"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - This field enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Counter always incremented between blocks"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Do not increment counter between blocks"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - This field enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter always incremented between blocks"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Do not increment counter between blocks"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - This field enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, AesCtrNoIncrementSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "This register enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctr_no_increment::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctr_no_increment::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtrNoIncrementSpec;
impl crate::RegisterSpec for AesCtrNoIncrementSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ctr_no_increment::R`](R) reader structure"]
impl crate::Readable for AesCtrNoIncrementSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctr_no_increment::W`](W) writer structure"]
impl crate::Writable for AesCtrNoIncrementSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CTR_NO_INCREMENT to value 0"]
impl crate::Resettable for AesCtrNoIncrementSpec {}
