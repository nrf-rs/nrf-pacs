#[doc = "Register `BOOTLOADERADDR` reader"]
pub struct R(crate::R<BOOTLOADERADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTLOADERADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTLOADERADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTLOADERADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTLOADERADDR` writer"]
pub struct W(crate::W<BOOTLOADERADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTLOADERADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BOOTLOADERADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTLOADERADDR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader start address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootloaderaddr](index.html) module"]
pub struct BOOTLOADERADDR_SPEC;
impl crate::RegisterSpec for BOOTLOADERADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootloaderaddr::R](R) reader structure"]
impl crate::Readable for BOOTLOADERADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootloaderaddr::W](W) writer structure"]
impl crate::Writable for BOOTLOADERADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOTLOADERADDR to value 0xffff_ffff"]
impl crate::Resettable for BOOTLOADERADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
