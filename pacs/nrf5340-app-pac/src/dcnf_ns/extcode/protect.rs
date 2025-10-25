#[doc = "Register `PROTECT` reader"]
pub type R = crate::R<ProtectSpec>;
#[doc = "Register `PROTECT` writer"]
pub type W = crate::W<ProtectSpec>;
#[doc = "Control access to slave 0 of master EXTCODE\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave0 {
    #[doc = "0: Access to slave is allowed"]
    Allowed = 0,
    #[doc = "1: Access to slave is blocked"]
    Blocked = 1,
}
impl From<Slave0> for bool {
    #[inline(always)]
    fn from(variant: Slave0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE0` reader - Control access to slave 0 of master EXTCODE\\[n\\]"]
pub type Slave0R = crate::BitReader<Slave0>;
impl Slave0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave0 {
        match self.bits {
            false => Slave0::Allowed,
            true => Slave0::Blocked,
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Slave0::Allowed
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Slave0::Blocked
    }
}
#[doc = "Field `SLAVE0` writer - Control access to slave 0 of master EXTCODE\\[n\\]"]
pub type Slave0W<'a, REG> = crate::BitWriter<'a, REG, Slave0>;
impl<'a, REG> Slave0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Slave0::Allowed)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Slave0::Blocked)
    }
}
impl R {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> Slave0R {
        Slave0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTCODE\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> Slave0W<'_, ProtectSpec> {
        Slave0W::new(self, 0)
    }
}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`protect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectSpec;
impl crate::RegisterSpec for ProtectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protect::R`](R) reader structure"]
impl crate::Readable for ProtectSpec {}
#[doc = "`write(|w| ..)` method takes [`protect::W`](W) writer structure"]
impl crate::Writable for ProtectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROTECT to value 0"]
impl crate::Resettable for ProtectSpec {}
