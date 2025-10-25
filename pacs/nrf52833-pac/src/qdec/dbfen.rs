#[doc = "Register `DBFEN` reader"]
pub type R = crate::R<DbfenSpec>;
#[doc = "Register `DBFEN` writer"]
pub type W = crate::W<DbfenSpec>;
#[doc = "Enable input debounce filters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbfen {
    #[doc = "0: Debounce input filters disabled"]
    Disabled = 0,
    #[doc = "1: Debounce input filters enabled"]
    Enabled = 1,
}
impl From<Dbfen> for bool {
    #[inline(always)]
    fn from(variant: Dbfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBFEN` reader - Enable input debounce filters"]
pub type DbfenR = crate::BitReader<Dbfen>;
impl DbfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbfen {
        match self.bits {
            false => Dbfen::Disabled,
            true => Dbfen::Enabled,
        }
    }
    #[doc = "Debounce input filters disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dbfen::Disabled
    }
    #[doc = "Debounce input filters enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dbfen::Enabled
    }
}
#[doc = "Field `DBFEN` writer - Enable input debounce filters"]
pub type DbfenW<'a, REG> = crate::BitWriter<'a, REG, Dbfen>;
impl<'a, REG> DbfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debounce input filters disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dbfen::Disabled)
    }
    #[doc = "Debounce input filters enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dbfen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&self) -> DbfenR {
        DbfenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&mut self) -> DbfenW<'_, DbfenSpec> {
        DbfenW::new(self, 0)
    }
}
#[doc = "Enable input debounce filters\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbfenSpec;
impl crate::RegisterSpec for DbfenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbfen::R`](R) reader structure"]
impl crate::Readable for DbfenSpec {}
#[doc = "`write(|w| ..)` method takes [`dbfen::W`](W) writer structure"]
impl crate::Writable for DbfenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBFEN to value 0"]
impl crate::Resettable for DbfenSpec {}
