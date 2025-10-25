#[doc = "Register `LAR` reader"]
pub type R = crate::R<LarSpec>;
#[doc = "Register `LAR` writer"]
pub type W = crate::W<LarSpec>;
#[doc = "A write of 0xC5ACCE55 enables further write access to this device. Any other write removes write access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Access {
    #[doc = "3316436565: Unlock register interface."]
    UnLock = 3316436565,
}
impl From<Access> for u32 {
    #[inline(always)]
    fn from(variant: Access) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Access {
    type Ux = u32;
}
impl crate::IsEnum for Access {}
#[doc = "Field `ACCESS` reader - A write of 0xC5ACCE55 enables further write access to this device. Any other write removes write access."]
pub type AccessR = crate::FieldReader<Access>;
impl AccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Access> {
        match self.bits {
            3316436565 => Some(Access::UnLock),
            _ => None,
        }
    }
    #[doc = "Unlock register interface."]
    #[inline(always)]
    pub fn is_un_lock(&self) -> bool {
        *self == Access::UnLock
    }
}
#[doc = "Field `ACCESS` writer - A write of 0xC5ACCE55 enables further write access to this device. Any other write removes write access."]
pub type AccessW<'a, REG> = crate::FieldWriter<'a, REG, 32, Access>;
impl<'a, REG> AccessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unlock register interface."]
    #[inline(always)]
    pub fn un_lock(self) -> &'a mut crate::W<REG> {
        self.variant(Access::UnLock)
    }
}
impl R {
    #[doc = "Bits 0:31 - A write of 0xC5ACCE55 enables further write access to this device. Any other write removes write access."]
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A write of 0xC5ACCE55 enables further write access to this device. Any other write removes write access."]
    #[inline(always)]
    pub fn access(&mut self) -> AccessW<'_, LarSpec> {
        AccessW::new(self, 0)
    }
}
#[doc = "This is used to enable write access to device registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`lar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LarSpec;
impl crate::RegisterSpec for LarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lar::R`](R) reader structure"]
impl crate::Readable for LarSpec {}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LarSpec {}
