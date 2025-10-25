#[doc = "Register `OTP[%s]` reader"]
pub type R = crate::R<OtpSpec>;
#[doc = "Register `OTP[%s]` writer"]
pub type W = crate::W<OtpSpec>;
#[doc = "Field `LOWER` reader - Lower half word"]
pub type LowerR = crate::FieldReader<u16>;
#[doc = "Field `LOWER` writer - Lower half word"]
pub type LowerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPPER` reader - Upper half word"]
pub type UpperR = crate::FieldReader<u16>;
#[doc = "Field `UPPER` writer - Upper half word"]
pub type UpperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lower half word"]
    #[inline(always)]
    pub fn lower(&self) -> LowerR {
        LowerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper half word"]
    #[inline(always)]
    pub fn upper(&self) -> UpperR {
        UpperR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower half word"]
    #[inline(always)]
    pub fn lower(&mut self) -> LowerW<'_, OtpSpec> {
        LowerW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Upper half word"]
    #[inline(always)]
    pub fn upper(&mut self) -> UpperW<'_, OtpSpec> {
        UpperW::new(self, 16)
    }
}
#[doc = "Description collection: One time programmable memory\n\nYou can [`read`](crate::Reg::read) this register and get [`otp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpSpec;
impl crate::RegisterSpec for OtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp::R`](R) reader structure"]
impl crate::Readable for OtpSpec {}
#[doc = "`write(|w| ..)` method takes [`otp::W`](W) writer structure"]
impl crate::Writable for OtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP[%s] to value 0xffff_ffff"]
impl crate::Resettable for OtpSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
