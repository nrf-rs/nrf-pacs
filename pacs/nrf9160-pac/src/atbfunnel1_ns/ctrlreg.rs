#[doc = "Register `CTRLREG` reader"]
pub type R = crate::R<CtrlregSpec>;
#[doc = "Register `CTRLREG` writer"]
pub type W = crate::W<CtrlregSpec>;
#[doc = "Enable slave port 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens0 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens0> for bool {
    #[inline(always)]
    fn from(variant: Ens0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_0` reader - Enable slave port 0."]
pub type Ens0R = crate::BitReader<Ens0>;
impl Ens0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens0 {
        match self.bits {
            false => Ens0::Disabled,
            true => Ens0::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens0::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens0::Enabled
    }
}
#[doc = "Field `ENS_0` writer - Enable slave port 0."]
pub type Ens0W<'a, REG> = crate::BitWriter<'a, REG, Ens0>;
impl<'a, REG> Ens0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens0::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens0::Enabled)
    }
}
#[doc = "Enable slave port 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens1 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens1> for bool {
    #[inline(always)]
    fn from(variant: Ens1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_1` reader - Enable slave port 1."]
pub type Ens1R = crate::BitReader<Ens1>;
impl Ens1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens1 {
        match self.bits {
            false => Ens1::Disabled,
            true => Ens1::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens1::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens1::Enabled
    }
}
#[doc = "Field `ENS_1` writer - Enable slave port 1."]
pub type Ens1W<'a, REG> = crate::BitWriter<'a, REG, Ens1>;
impl<'a, REG> Ens1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens1::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens1::Enabled)
    }
}
#[doc = "Enable slave port 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens2 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens2> for bool {
    #[inline(always)]
    fn from(variant: Ens2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_2` reader - Enable slave port 2."]
pub type Ens2R = crate::BitReader<Ens2>;
impl Ens2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens2 {
        match self.bits {
            false => Ens2::Disabled,
            true => Ens2::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens2::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens2::Enabled
    }
}
#[doc = "Field `ENS_2` writer - Enable slave port 2."]
pub type Ens2W<'a, REG> = crate::BitWriter<'a, REG, Ens2>;
impl<'a, REG> Ens2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens2::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens2::Enabled)
    }
}
#[doc = "Enable slave port 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens3 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens3> for bool {
    #[inline(always)]
    fn from(variant: Ens3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_3` reader - Enable slave port 3."]
pub type Ens3R = crate::BitReader<Ens3>;
impl Ens3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens3 {
        match self.bits {
            false => Ens3::Disabled,
            true => Ens3::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens3::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens3::Enabled
    }
}
#[doc = "Field `ENS_3` writer - Enable slave port 3."]
pub type Ens3W<'a, REG> = crate::BitWriter<'a, REG, Ens3>;
impl<'a, REG> Ens3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens3::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens3::Enabled)
    }
}
#[doc = "Enable slave port 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens4 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens4> for bool {
    #[inline(always)]
    fn from(variant: Ens4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_4` reader - Enable slave port 4."]
pub type Ens4R = crate::BitReader<Ens4>;
impl Ens4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens4 {
        match self.bits {
            false => Ens4::Disabled,
            true => Ens4::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens4::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens4::Enabled
    }
}
#[doc = "Field `ENS_4` writer - Enable slave port 4."]
pub type Ens4W<'a, REG> = crate::BitWriter<'a, REG, Ens4>;
impl<'a, REG> Ens4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens4::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens4::Enabled)
    }
}
#[doc = "Enable slave port 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens5 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens5> for bool {
    #[inline(always)]
    fn from(variant: Ens5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_5` reader - Enable slave port 5."]
pub type Ens5R = crate::BitReader<Ens5>;
impl Ens5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens5 {
        match self.bits {
            false => Ens5::Disabled,
            true => Ens5::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens5::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens5::Enabled
    }
}
#[doc = "Field `ENS_5` writer - Enable slave port 5."]
pub type Ens5W<'a, REG> = crate::BitWriter<'a, REG, Ens5>;
impl<'a, REG> Ens5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens5::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens5::Enabled)
    }
}
#[doc = "Enable slave port 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens6 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens6> for bool {
    #[inline(always)]
    fn from(variant: Ens6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_6` reader - Enable slave port 6."]
pub type Ens6R = crate::BitReader<Ens6>;
impl Ens6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens6 {
        match self.bits {
            false => Ens6::Disabled,
            true => Ens6::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens6::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens6::Enabled
    }
}
#[doc = "Field `ENS_6` writer - Enable slave port 6."]
pub type Ens6W<'a, REG> = crate::BitWriter<'a, REG, Ens6>;
impl<'a, REG> Ens6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens6::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens6::Enabled)
    }
}
#[doc = "Enable slave port 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ens7 {
    #[doc = "0: Slave port disabled. This excludes the port from the priority selection scheme."]
    Disabled = 0,
    #[doc = "1: Slave port enabled."]
    Enabled = 1,
}
impl From<Ens7> for bool {
    #[inline(always)]
    fn from(variant: Ens7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENS_7` reader - Enable slave port 7."]
pub type Ens7R = crate::BitReader<Ens7>;
impl Ens7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ens7 {
        match self.bits {
            false => Ens7::Disabled,
            true => Ens7::Enabled,
        }
    }
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ens7::Disabled
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ens7::Enabled
    }
}
#[doc = "Field `ENS_7` writer - Enable slave port 7."]
pub type Ens7W<'a, REG> = crate::BitWriter<'a, REG, Ens7>;
impl<'a, REG> Ens7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave port disabled. This excludes the port from the priority selection scheme."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens7::Disabled)
    }
    #[doc = "Slave port enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ens7::Enabled)
    }
}
#[doc = "Field `HT` reader - Hold Time. The formatting scheme can become inefficient when fast switching occurs, and you can use this setting to minimize switching. When a source has nothing to transmit, then another source is selected irrespective of the minimum number of transactions. The ATB funnel holds for the minimum hold time and one additional transaction. The actual hold time is the register value plus 1. The maximum value that can be entered is 0b1110 and this equates to 15 transactions. 0b1111 is reserved."]
pub type HtR = crate::FieldReader;
#[doc = "Field `HT` writer - Hold Time. The formatting scheme can become inefficient when fast switching occurs, and you can use this setting to minimize switching. When a source has nothing to transmit, then another source is selected irrespective of the minimum number of transactions. The ATB funnel holds for the minimum hold time and one additional transaction. The actual hold time is the register value plus 1. The maximum value that can be entered is 0b1110 and this equates to 15 transactions. 0b1111 is reserved."]
pub type HtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable slave port 0."]
    #[inline(always)]
    pub fn ens_0(&self) -> Ens0R {
        Ens0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable slave port 1."]
    #[inline(always)]
    pub fn ens_1(&self) -> Ens1R {
        Ens1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable slave port 2."]
    #[inline(always)]
    pub fn ens_2(&self) -> Ens2R {
        Ens2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable slave port 3."]
    #[inline(always)]
    pub fn ens_3(&self) -> Ens3R {
        Ens3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable slave port 4."]
    #[inline(always)]
    pub fn ens_4(&self) -> Ens4R {
        Ens4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable slave port 5."]
    #[inline(always)]
    pub fn ens_5(&self) -> Ens5R {
        Ens5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable slave port 6."]
    #[inline(always)]
    pub fn ens_6(&self) -> Ens6R {
        Ens6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable slave port 7."]
    #[inline(always)]
    pub fn ens_7(&self) -> Ens7R {
        Ens7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Hold Time. The formatting scheme can become inefficient when fast switching occurs, and you can use this setting to minimize switching. When a source has nothing to transmit, then another source is selected irrespective of the minimum number of transactions. The ATB funnel holds for the minimum hold time and one additional transaction. The actual hold time is the register value plus 1. The maximum value that can be entered is 0b1110 and this equates to 15 transactions. 0b1111 is reserved."]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable slave port 0."]
    #[inline(always)]
    pub fn ens_0(&mut self) -> Ens0W<'_, CtrlregSpec> {
        Ens0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable slave port 1."]
    #[inline(always)]
    pub fn ens_1(&mut self) -> Ens1W<'_, CtrlregSpec> {
        Ens1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable slave port 2."]
    #[inline(always)]
    pub fn ens_2(&mut self) -> Ens2W<'_, CtrlregSpec> {
        Ens2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable slave port 3."]
    #[inline(always)]
    pub fn ens_3(&mut self) -> Ens3W<'_, CtrlregSpec> {
        Ens3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable slave port 4."]
    #[inline(always)]
    pub fn ens_4(&mut self) -> Ens4W<'_, CtrlregSpec> {
        Ens4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable slave port 5."]
    #[inline(always)]
    pub fn ens_5(&mut self) -> Ens5W<'_, CtrlregSpec> {
        Ens5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable slave port 6."]
    #[inline(always)]
    pub fn ens_6(&mut self) -> Ens6W<'_, CtrlregSpec> {
        Ens6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable slave port 7."]
    #[inline(always)]
    pub fn ens_7(&mut self) -> Ens7W<'_, CtrlregSpec> {
        Ens7W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Hold Time. The formatting scheme can become inefficient when fast switching occurs, and you can use this setting to minimize switching. When a source has nothing to transmit, then another source is selected irrespective of the minimum number of transactions. The ATB funnel holds for the minimum hold time and one additional transaction. The actual hold time is the register value plus 1. The maximum value that can be entered is 0b1110 and this equates to 15 transactions. 0b1111 is reserved."]
    #[inline(always)]
    pub fn ht(&mut self) -> HtW<'_, CtrlregSpec> {
        HtW::new(self, 8)
    }
}
#[doc = "The IDFILTER0 register enables the programming of ID filtering for master port 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlregSpec;
impl crate::RegisterSpec for CtrlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlreg::R`](R) reader structure"]
impl crate::Readable for CtrlregSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlreg::W`](W) writer structure"]
impl crate::Writable for CtrlregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLREG to value 0"]
impl crate::Resettable for CtrlregSpec {}
