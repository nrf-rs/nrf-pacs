#[doc = "Register `CTIOUTEN[%s]` reader"]
pub type R = crate::R<CtioutenSpec>;
#[doc = "Register `CTIOUTEN[%s]` writer"]
pub type W = crate::W<CtioutenSpec>;
#[doc = "Enables a cross trigger event to ctitrigout when channel 0 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigouten0 {
    #[doc = "0: Channel 0 is ignored by output trigger n."]
    Disabled = 0,
    #[doc = "1: When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    Enabled = 1,
}
impl From<Trigouten0> for bool {
    #[inline(always)]
    fn from(variant: Trigouten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_0` reader - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
pub type Trigouten0R = crate::BitReader<Trigouten0>;
impl Trigouten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigouten0 {
        match self.bits {
            false => Trigouten0::Disabled,
            true => Trigouten0::Enabled,
        }
    }
    #[doc = "Channel 0 is ignored by output trigger n."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigouten0::Disabled
    }
    #[doc = "When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigouten0::Enabled
    }
}
#[doc = "Field `TRIGOUTEN_0` writer - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
pub type Trigouten0W<'a, REG> = crate::BitWriter<'a, REG, Trigouten0>;
impl<'a, REG> Trigouten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten0::Disabled)
    }
    #[doc = "When an event occurs on channel 0, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten0::Enabled)
    }
}
#[doc = "Enables a cross trigger event to ctitrigout when channel 1 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigouten1 {
    #[doc = "0: Channel 1 is ignored by output trigger n."]
    Disabled = 0,
    #[doc = "1: When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    Enabled = 1,
}
impl From<Trigouten1> for bool {
    #[inline(always)]
    fn from(variant: Trigouten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_1` reader - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
pub type Trigouten1R = crate::BitReader<Trigouten1>;
impl Trigouten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigouten1 {
        match self.bits {
            false => Trigouten1::Disabled,
            true => Trigouten1::Enabled,
        }
    }
    #[doc = "Channel 1 is ignored by output trigger n."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigouten1::Disabled
    }
    #[doc = "When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigouten1::Enabled
    }
}
#[doc = "Field `TRIGOUTEN_1` writer - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
pub type Trigouten1W<'a, REG> = crate::BitWriter<'a, REG, Trigouten1>;
impl<'a, REG> Trigouten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten1::Disabled)
    }
    #[doc = "When an event occurs on channel 1, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten1::Enabled)
    }
}
#[doc = "Enables a cross trigger event to ctitrigout when channel 2 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigouten2 {
    #[doc = "0: Channel 2 is ignored by output trigger n."]
    Disabled = 0,
    #[doc = "1: When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    Enabled = 1,
}
impl From<Trigouten2> for bool {
    #[inline(always)]
    fn from(variant: Trigouten2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_2` reader - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
pub type Trigouten2R = crate::BitReader<Trigouten2>;
impl Trigouten2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigouten2 {
        match self.bits {
            false => Trigouten2::Disabled,
            true => Trigouten2::Enabled,
        }
    }
    #[doc = "Channel 2 is ignored by output trigger n."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigouten2::Disabled
    }
    #[doc = "When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigouten2::Enabled
    }
}
#[doc = "Field `TRIGOUTEN_2` writer - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
pub type Trigouten2W<'a, REG> = crate::BitWriter<'a, REG, Trigouten2>;
impl<'a, REG> Trigouten2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten2::Disabled)
    }
    #[doc = "When an event occurs on channel 2, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten2::Enabled)
    }
}
#[doc = "Enables a cross trigger event to ctitrigout when channel 3 is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigouten3 {
    #[doc = "0: Channel 3 is ignored by output trigger n."]
    Disabled = 0,
    #[doc = "1: When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    Enabled = 1,
}
impl From<Trigouten3> for bool {
    #[inline(always)]
    fn from(variant: Trigouten3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGOUTEN_3` reader - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
pub type Trigouten3R = crate::BitReader<Trigouten3>;
impl Trigouten3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigouten3 {
        match self.bits {
            false => Trigouten3::Disabled,
            true => Trigouten3::Enabled,
        }
    }
    #[doc = "Channel 3 is ignored by output trigger n."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigouten3::Disabled
    }
    #[doc = "When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigouten3::Enabled
    }
}
#[doc = "Field `TRIGOUTEN_3` writer - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
pub type Trigouten3W<'a, REG> = crate::BitWriter<'a, REG, Trigouten3>;
impl<'a, REG> Trigouten3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 is ignored by output trigger n."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten3::Disabled)
    }
    #[doc = "When an event occurs on channel 3, generate an event on output event n (ctitrigout\\[n\\])."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigouten3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub fn trigouten_0(&self) -> Trigouten0R {
        Trigouten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub fn trigouten_1(&self) -> Trigouten1R {
        Trigouten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub fn trigouten_2(&self) -> Trigouten2R {
        Trigouten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub fn trigouten_3(&self) -> Trigouten3R {
        Trigouten3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 is activated."]
    #[inline(always)]
    pub fn trigouten_0(&mut self) -> Trigouten0W<'_, CtioutenSpec> {
        Trigouten0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 is activated."]
    #[inline(always)]
    pub fn trigouten_1(&mut self) -> Trigouten1W<'_, CtioutenSpec> {
        Trigouten1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 is activated."]
    #[inline(always)]
    pub fn trigouten_2(&mut self) -> Trigouten2W<'_, CtioutenSpec> {
        Trigouten2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 is activated."]
    #[inline(always)]
    pub fn trigouten_3(&mut self) -> Trigouten3W<'_, CtioutenSpec> {
        Trigouten3W::new(self, 3)
    }
}
#[doc = "Description collection: CTI Trigger output\n\nYou can [`read`](crate::Reg::read) this register and get [`ctiouten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiouten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtioutenSpec;
impl crate::RegisterSpec for CtioutenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctiouten::R`](R) reader structure"]
impl crate::Readable for CtioutenSpec {}
#[doc = "`write(|w| ..)` method takes [`ctiouten::W`](W) writer structure"]
impl crate::Writable for CtioutenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIOUTEN[%s] to value 0"]
impl crate::Resettable for CtioutenSpec {}
