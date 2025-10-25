#[doc = "Register `SUPPORTEDTRIGGERMODES` reader"]
pub type R = crate::R<SupportedtriggermodesSpec>;
#[doc = "Register `SUPPORTEDTRIGGERMODES` writer"]
pub type W = crate::W<SupportedtriggermodesSpec>;
#[doc = "Indicates whether multiplying the trigger counter by 2^(0+1) is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult0 {
    #[doc = "0: Multiplying the trigger counter by 2^(0+1) is supported."]
    NotSelected = 0,
    #[doc = "1: Multiplying the trigger counter by 2^(0+1) is supported."]
    Selected = 1,
}
impl From<Mult0> for bool {
    #[inline(always)]
    fn from(variant: Mult0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_0` reader - Indicates whether multiplying the trigger counter by 2^(0+1) is supported."]
pub type Mult0R = crate::BitReader<Mult0>;
impl Mult0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult0 {
        match self.bits {
            false => Mult0::NotSelected,
            true => Mult0::Selected,
        }
    }
    #[doc = "Multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Mult0::NotSelected
    }
    #[doc = "Multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Mult0::Selected
    }
}
#[doc = "Field `MULT_0` writer - Indicates whether multiplying the trigger counter by 2^(0+1) is supported."]
pub type Mult0W<'a, REG> = crate::BitWriter<'a, REG, Mult0>;
impl<'a, REG> Mult0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult0::NotSelected)
    }
    #[doc = "Multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult0::Selected)
    }
}
#[doc = "Indicates whether multiplying the trigger counter by 2^(1+1) is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult1 {
    #[doc = "0: Multiplying the trigger counter by 2^(1+1) is supported."]
    NotSelected = 0,
    #[doc = "1: Multiplying the trigger counter by 2^(1+1) is supported."]
    Selected = 1,
}
impl From<Mult1> for bool {
    #[inline(always)]
    fn from(variant: Mult1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_1` reader - Indicates whether multiplying the trigger counter by 2^(1+1) is supported."]
pub type Mult1R = crate::BitReader<Mult1>;
impl Mult1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult1 {
        match self.bits {
            false => Mult1::NotSelected,
            true => Mult1::Selected,
        }
    }
    #[doc = "Multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Mult1::NotSelected
    }
    #[doc = "Multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Mult1::Selected
    }
}
#[doc = "Field `MULT_1` writer - Indicates whether multiplying the trigger counter by 2^(1+1) is supported."]
pub type Mult1W<'a, REG> = crate::BitWriter<'a, REG, Mult1>;
impl<'a, REG> Mult1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult1::NotSelected)
    }
    #[doc = "Multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult1::Selected)
    }
}
#[doc = "Indicates whether multiplying the trigger counter by 2^(2+1) is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult2 {
    #[doc = "0: Multiplying the trigger counter by 2^(2+1) is supported."]
    NotSelected = 0,
    #[doc = "1: Multiplying the trigger counter by 2^(2+1) is supported."]
    Selected = 1,
}
impl From<Mult2> for bool {
    #[inline(always)]
    fn from(variant: Mult2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_2` reader - Indicates whether multiplying the trigger counter by 2^(2+1) is supported."]
pub type Mult2R = crate::BitReader<Mult2>;
impl Mult2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult2 {
        match self.bits {
            false => Mult2::NotSelected,
            true => Mult2::Selected,
        }
    }
    #[doc = "Multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Mult2::NotSelected
    }
    #[doc = "Multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Mult2::Selected
    }
}
#[doc = "Field `MULT_2` writer - Indicates whether multiplying the trigger counter by 2^(2+1) is supported."]
pub type Mult2W<'a, REG> = crate::BitWriter<'a, REG, Mult2>;
impl<'a, REG> Mult2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult2::NotSelected)
    }
    #[doc = "Multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult2::Selected)
    }
}
#[doc = "Indicates whether multiplying the trigger counter by 2^(3+1) is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult3 {
    #[doc = "0: Multiplying the trigger counter by 2^(3+1) is supported."]
    NotSelected = 0,
    #[doc = "1: Multiplying the trigger counter by 2^(3+1) is supported."]
    Selected = 1,
}
impl From<Mult3> for bool {
    #[inline(always)]
    fn from(variant: Mult3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_3` reader - Indicates whether multiplying the trigger counter by 2^(3+1) is supported."]
pub type Mult3R = crate::BitReader<Mult3>;
impl Mult3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult3 {
        match self.bits {
            false => Mult3::NotSelected,
            true => Mult3::Selected,
        }
    }
    #[doc = "Multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Mult3::NotSelected
    }
    #[doc = "Multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Mult3::Selected
    }
}
#[doc = "Field `MULT_3` writer - Indicates whether multiplying the trigger counter by 2^(3+1) is supported."]
pub type Mult3W<'a, REG> = crate::BitWriter<'a, REG, Mult3>;
impl<'a, REG> Mult3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult3::NotSelected)
    }
    #[doc = "Multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult3::Selected)
    }
}
#[doc = "Indicates whether multiplying the trigger counter by 2^(4+1) is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mult4 {
    #[doc = "0: Multiplying the trigger counter by 2^(4+1) is supported."]
    NotSelected = 0,
    #[doc = "1: Multiplying the trigger counter by 2^(4+1) is supported."]
    Selected = 1,
}
impl From<Mult4> for bool {
    #[inline(always)]
    fn from(variant: Mult4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULT_4` reader - Indicates whether multiplying the trigger counter by 2^(4+1) is supported."]
pub type Mult4R = crate::BitReader<Mult4>;
impl Mult4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mult4 {
        match self.bits {
            false => Mult4::NotSelected,
            true => Mult4::Selected,
        }
    }
    #[doc = "Multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Mult4::NotSelected
    }
    #[doc = "Multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == Mult4::Selected
    }
}
#[doc = "Field `MULT_4` writer - Indicates whether multiplying the trigger counter by 2^(4+1) is supported."]
pub type Mult4W<'a, REG> = crate::BitWriter<'a, REG, Mult4>;
impl<'a, REG> Mult4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult4::NotSelected)
    }
    #[doc = "Multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(Mult4::Selected)
    }
}
#[doc = "Indicates whether an 8-bit wide counter register is implemented.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcount8 {
    #[doc = "0: An 8-bit wide counter register is implemented."]
    NotImplemented = 0,
    #[doc = "1: An 8-bit wide counter register is implemented."]
    Implemented = 1,
}
impl From<Tcount8> for bool {
    #[inline(always)]
    fn from(variant: Tcount8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCOUNT8` reader - Indicates whether an 8-bit wide counter register is implemented."]
pub type Tcount8R = crate::BitReader<Tcount8>;
impl Tcount8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcount8 {
        match self.bits {
            false => Tcount8::NotImplemented,
            true => Tcount8::Implemented,
        }
    }
    #[doc = "An 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Tcount8::NotImplemented
    }
    #[doc = "An 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Tcount8::Implemented
    }
}
#[doc = "Field `TCOUNT8` writer - Indicates whether an 8-bit wide counter register is implemented."]
pub type Tcount8W<'a, REG> = crate::BitWriter<'a, REG, Tcount8>;
impl<'a, REG> Tcount8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Tcount8::NotImplemented)
    }
    #[doc = "An 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Tcount8::Implemented)
    }
}
#[doc = "A trigger has occurred and the counter has reached 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered {
    #[doc = "0: Trigger has not occurred."]
    NotOccured = 0,
    #[doc = "1: Trigger has occurred."]
    Occured = 1,
}
impl From<Triggered> for bool {
    #[inline(always)]
    fn from(variant: Triggered) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED` reader - A trigger has occurred and the counter has reached 0."]
pub type TriggeredR = crate::BitReader<Triggered>;
impl TriggeredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered {
        match self.bits {
            false => Triggered::NotOccured,
            true => Triggered::Occured,
        }
    }
    #[doc = "Trigger has not occurred."]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == Triggered::NotOccured
    }
    #[doc = "Trigger has occurred."]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == Triggered::Occured
    }
}
#[doc = "Field `TRIGGERED` writer - A trigger has occurred and the counter has reached 0."]
pub type TriggeredW<'a, REG> = crate::BitWriter<'a, REG, Triggered>;
impl<'a, REG> TriggeredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger has not occurred."]
    #[inline(always)]
    pub fn not_occured(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered::NotOccured)
    }
    #[doc = "Trigger has occurred."]
    #[inline(always)]
    pub fn occured(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered::Occured)
    }
}
#[doc = "A trigger has occurred but the counter is not at 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgrun {
    #[doc = "0: Either a trigger has not occurred or the counter is at 0."]
    NotOccured = 0,
    #[doc = "1: A trigger has occurred but the counter is not at 0."]
    Occured = 1,
}
impl From<Trgrun> for bool {
    #[inline(always)]
    fn from(variant: Trgrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGRUN` reader - A trigger has occurred but the counter is not at 0."]
pub type TrgrunR = crate::BitReader<Trgrun>;
impl TrgrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgrun {
        match self.bits {
            false => Trgrun::NotOccured,
            true => Trgrun::Occured,
        }
    }
    #[doc = "Either a trigger has not occurred or the counter is at 0."]
    #[inline(always)]
    pub fn is_not_occured(&self) -> bool {
        *self == Trgrun::NotOccured
    }
    #[doc = "A trigger has occurred but the counter is not at 0."]
    #[inline(always)]
    pub fn is_occured(&self) -> bool {
        *self == Trgrun::Occured
    }
}
#[doc = "Field `TRGRUN` writer - A trigger has occurred but the counter is not at 0."]
pub type TrgrunW<'a, REG> = crate::BitWriter<'a, REG, Trgrun>;
impl<'a, REG> TrgrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Either a trigger has not occurred or the counter is at 0."]
    #[inline(always)]
    pub fn not_occured(self) -> &'a mut crate::W<REG> {
        self.variant(Trgrun::NotOccured)
    }
    #[doc = "A trigger has occurred but the counter is not at 0."]
    #[inline(always)]
    pub fn occured(self) -> &'a mut crate::W<REG> {
        self.variant(Trgrun::Occured)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn mult_0(&self) -> Mult0R {
        Mult0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn mult_1(&self) -> Mult1R {
        Mult1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn mult_2(&self) -> Mult2R {
        Mult2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates whether multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn mult_3(&self) -> Mult3R {
        Mult3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates whether multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn mult_4(&self) -> Mult4R {
        Mult4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates whether an 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn tcount8(&self) -> Tcount8R {
        Tcount8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - A trigger has occurred and the counter has reached 0."]
    #[inline(always)]
    pub fn triggered(&self) -> TriggeredR {
        TriggeredR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A trigger has occurred but the counter is not at 0."]
    #[inline(always)]
    pub fn trgrun(&self) -> TrgrunR {
        TrgrunR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether multiplying the trigger counter by 2^(0+1) is supported."]
    #[inline(always)]
    pub fn mult_0(&mut self) -> Mult0W<'_, SupportedtriggermodesSpec> {
        Mult0W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates whether multiplying the trigger counter by 2^(1+1) is supported."]
    #[inline(always)]
    pub fn mult_1(&mut self) -> Mult1W<'_, SupportedtriggermodesSpec> {
        Mult1W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates whether multiplying the trigger counter by 2^(2+1) is supported."]
    #[inline(always)]
    pub fn mult_2(&mut self) -> Mult2W<'_, SupportedtriggermodesSpec> {
        Mult2W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates whether multiplying the trigger counter by 2^(3+1) is supported."]
    #[inline(always)]
    pub fn mult_3(&mut self) -> Mult3W<'_, SupportedtriggermodesSpec> {
        Mult3W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates whether multiplying the trigger counter by 2^(4+1) is supported."]
    #[inline(always)]
    pub fn mult_4(&mut self) -> Mult4W<'_, SupportedtriggermodesSpec> {
        Mult4W::new(self, 4)
    }
    #[doc = "Bit 8 - Indicates whether an 8-bit wide counter register is implemented."]
    #[inline(always)]
    pub fn tcount8(&mut self) -> Tcount8W<'_, SupportedtriggermodesSpec> {
        Tcount8W::new(self, 8)
    }
    #[doc = "Bit 16 - A trigger has occurred and the counter has reached 0."]
    #[inline(always)]
    pub fn triggered(&mut self) -> TriggeredW<'_, SupportedtriggermodesSpec> {
        TriggeredW::new(self, 16)
    }
    #[doc = "Bit 17 - A trigger has occurred but the counter is not at 0."]
    #[inline(always)]
    pub fn trgrun(&mut self) -> TrgrunW<'_, SupportedtriggermodesSpec> {
        TrgrunW::new(self, 17)
    }
}
#[doc = "The Supported_trigger_modes register indicates the implemented trigger counter multipliers and other supported features of the trigger system.\n\nYou can [`read`](crate::Reg::read) this register and get [`supportedtriggermodes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supportedtriggermodes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SupportedtriggermodesSpec;
impl crate::RegisterSpec for SupportedtriggermodesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`supportedtriggermodes::R`](R) reader structure"]
impl crate::Readable for SupportedtriggermodesSpec {}
#[doc = "`write(|w| ..)` method takes [`supportedtriggermodes::W`](W) writer structure"]
impl crate::Writable for SupportedtriggermodesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUPPORTEDTRIGGERMODES to value 0"]
impl crate::Resettable for SupportedtriggermodesSpec {}
