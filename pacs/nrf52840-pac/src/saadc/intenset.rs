#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Started {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Started> for bool {
    #[inline(always)]
    fn from(variant: Started) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Write '1' to enable interrupt for event STARTED"]
pub type StartedR = crate::BitReader<Started>;
impl StartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Started {
        match self.bits {
            false => Started::Disabled,
            true => Started::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Started::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Started::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StartedWO> for bool {
    #[inline(always)]
    fn from(variant: StartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to enable interrupt for event STARTED"]
pub type StartedW<'a, REG> = crate::BitWriter<'a, REG, StartedWO>;
impl<'a, REG> StartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StartedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Write '1' to enable interrupt for event END"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::Disabled,
            true => End::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == End::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == End::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to enable interrupt for event END"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to enable interrupt for event DONE"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Disabled,
            true => Done::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DoneWO> for bool {
    #[inline(always)]
    fn from(variant: DoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to enable interrupt for event DONE"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, DoneWO>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DoneWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resultdone {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Resultdone> for bool {
    #[inline(always)]
    fn from(variant: Resultdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULTDONE` reader - Write '1' to enable interrupt for event RESULTDONE"]
pub type ResultdoneR = crate::BitReader<Resultdone>;
impl ResultdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resultdone {
        match self.bits {
            false => Resultdone::Disabled,
            true => Resultdone::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Resultdone::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Resultdone::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultdoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ResultdoneWO> for bool {
    #[inline(always)]
    fn from(variant: ResultdoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULTDONE` writer - Write '1' to enable interrupt for event RESULTDONE"]
pub type ResultdoneW<'a, REG> = crate::BitWriter<'a, REG, ResultdoneWO>;
impl<'a, REG> ResultdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ResultdoneWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calibratedone {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Calibratedone> for bool {
    #[inline(always)]
    fn from(variant: Calibratedone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIBRATEDONE` reader - Write '1' to enable interrupt for event CALIBRATEDONE"]
pub type CalibratedoneR = crate::BitReader<Calibratedone>;
impl CalibratedoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calibratedone {
        match self.bits {
            false => Calibratedone::Disabled,
            true => Calibratedone::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Calibratedone::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Calibratedone::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalibratedoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CalibratedoneWO> for bool {
    #[inline(always)]
    fn from(variant: CalibratedoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIBRATEDONE` writer - Write '1' to enable interrupt for event CALIBRATEDONE"]
pub type CalibratedoneW<'a, REG> = crate::BitWriter<'a, REG, CalibratedoneWO>;
impl<'a, REG> CalibratedoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CalibratedoneWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch0limith> for bool {
    #[inline(always)]
    fn from(variant: Ch0limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITH` reader - Write '1' to enable interrupt for event CH0LIMITH"]
pub type Ch0limithR = crate::BitReader<Ch0limith>;
impl Ch0limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0limith {
        match self.bits {
            false => Ch0limith::Disabled,
            true => Ch0limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch0limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch0limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITH` writer - Write '1' to enable interrupt for event CH0LIMITH"]
pub type Ch0limithW<'a, REG> = crate::BitWriter<'a, REG, Ch0limithWO>;
impl<'a, REG> Ch0limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch0limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch0limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITL` reader - Write '1' to enable interrupt for event CH0LIMITL"]
pub type Ch0limitlR = crate::BitReader<Ch0limitl>;
impl Ch0limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0limitl {
        match self.bits {
            false => Ch0limitl::Disabled,
            true => Ch0limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch0limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch0limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITL` writer - Write '1' to enable interrupt for event CH0LIMITL"]
pub type Ch0limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch0limitlWO>;
impl<'a, REG> Ch0limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch1limith> for bool {
    #[inline(always)]
    fn from(variant: Ch1limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITH` reader - Write '1' to enable interrupt for event CH1LIMITH"]
pub type Ch1limithR = crate::BitReader<Ch1limith>;
impl Ch1limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1limith {
        match self.bits {
            false => Ch1limith::Disabled,
            true => Ch1limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch1limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch1limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITH` writer - Write '1' to enable interrupt for event CH1LIMITH"]
pub type Ch1limithW<'a, REG> = crate::BitWriter<'a, REG, Ch1limithWO>;
impl<'a, REG> Ch1limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch1limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch1limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITL` reader - Write '1' to enable interrupt for event CH1LIMITL"]
pub type Ch1limitlR = crate::BitReader<Ch1limitl>;
impl Ch1limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1limitl {
        match self.bits {
            false => Ch1limitl::Disabled,
            true => Ch1limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch1limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch1limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITL` writer - Write '1' to enable interrupt for event CH1LIMITL"]
pub type Ch1limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch1limitlWO>;
impl<'a, REG> Ch1limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch2limith> for bool {
    #[inline(always)]
    fn from(variant: Ch2limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITH` reader - Write '1' to enable interrupt for event CH2LIMITH"]
pub type Ch2limithR = crate::BitReader<Ch2limith>;
impl Ch2limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2limith {
        match self.bits {
            false => Ch2limith::Disabled,
            true => Ch2limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch2limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch2limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITH` writer - Write '1' to enable interrupt for event CH2LIMITH"]
pub type Ch2limithW<'a, REG> = crate::BitWriter<'a, REG, Ch2limithWO>;
impl<'a, REG> Ch2limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch2limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch2limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITL` reader - Write '1' to enable interrupt for event CH2LIMITL"]
pub type Ch2limitlR = crate::BitReader<Ch2limitl>;
impl Ch2limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2limitl {
        match self.bits {
            false => Ch2limitl::Disabled,
            true => Ch2limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch2limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch2limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITL` writer - Write '1' to enable interrupt for event CH2LIMITL"]
pub type Ch2limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch2limitlWO>;
impl<'a, REG> Ch2limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch3limith> for bool {
    #[inline(always)]
    fn from(variant: Ch3limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITH` reader - Write '1' to enable interrupt for event CH3LIMITH"]
pub type Ch3limithR = crate::BitReader<Ch3limith>;
impl Ch3limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3limith {
        match self.bits {
            false => Ch3limith::Disabled,
            true => Ch3limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch3limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch3limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITH` writer - Write '1' to enable interrupt for event CH3LIMITH"]
pub type Ch3limithW<'a, REG> = crate::BitWriter<'a, REG, Ch3limithWO>;
impl<'a, REG> Ch3limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch3limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch3limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITL` reader - Write '1' to enable interrupt for event CH3LIMITL"]
pub type Ch3limitlR = crate::BitReader<Ch3limitl>;
impl Ch3limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3limitl {
        match self.bits {
            false => Ch3limitl::Disabled,
            true => Ch3limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch3limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch3limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITL` writer - Write '1' to enable interrupt for event CH3LIMITL"]
pub type Ch3limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch3limitlWO>;
impl<'a, REG> Ch3limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch4limith> for bool {
    #[inline(always)]
    fn from(variant: Ch4limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITH` reader - Write '1' to enable interrupt for event CH4LIMITH"]
pub type Ch4limithR = crate::BitReader<Ch4limith>;
impl Ch4limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4limith {
        match self.bits {
            false => Ch4limith::Disabled,
            true => Ch4limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch4limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch4limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITH` writer - Write '1' to enable interrupt for event CH4LIMITH"]
pub type Ch4limithW<'a, REG> = crate::BitWriter<'a, REG, Ch4limithWO>;
impl<'a, REG> Ch4limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch4limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch4limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITL` reader - Write '1' to enable interrupt for event CH4LIMITL"]
pub type Ch4limitlR = crate::BitReader<Ch4limitl>;
impl Ch4limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4limitl {
        match self.bits {
            false => Ch4limitl::Disabled,
            true => Ch4limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch4limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch4limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITL` writer - Write '1' to enable interrupt for event CH4LIMITL"]
pub type Ch4limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch4limitlWO>;
impl<'a, REG> Ch4limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch5limith> for bool {
    #[inline(always)]
    fn from(variant: Ch5limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITH` reader - Write '1' to enable interrupt for event CH5LIMITH"]
pub type Ch5limithR = crate::BitReader<Ch5limith>;
impl Ch5limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5limith {
        match self.bits {
            false => Ch5limith::Disabled,
            true => Ch5limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch5limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch5limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITH` writer - Write '1' to enable interrupt for event CH5LIMITH"]
pub type Ch5limithW<'a, REG> = crate::BitWriter<'a, REG, Ch5limithWO>;
impl<'a, REG> Ch5limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch5limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch5limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITL` reader - Write '1' to enable interrupt for event CH5LIMITL"]
pub type Ch5limitlR = crate::BitReader<Ch5limitl>;
impl Ch5limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5limitl {
        match self.bits {
            false => Ch5limitl::Disabled,
            true => Ch5limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch5limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch5limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITL` writer - Write '1' to enable interrupt for event CH5LIMITL"]
pub type Ch5limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch5limitlWO>;
impl<'a, REG> Ch5limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch6limith> for bool {
    #[inline(always)]
    fn from(variant: Ch6limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITH` reader - Write '1' to enable interrupt for event CH6LIMITH"]
pub type Ch6limithR = crate::BitReader<Ch6limith>;
impl Ch6limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6limith {
        match self.bits {
            false => Ch6limith::Disabled,
            true => Ch6limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch6limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch6limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITH` writer - Write '1' to enable interrupt for event CH6LIMITH"]
pub type Ch6limithW<'a, REG> = crate::BitWriter<'a, REG, Ch6limithWO>;
impl<'a, REG> Ch6limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch6limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch6limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITL` reader - Write '1' to enable interrupt for event CH6LIMITL"]
pub type Ch6limitlR = crate::BitReader<Ch6limitl>;
impl Ch6limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6limitl {
        match self.bits {
            false => Ch6limitl::Disabled,
            true => Ch6limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch6limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch6limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITL` writer - Write '1' to enable interrupt for event CH6LIMITL"]
pub type Ch6limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch6limitlWO>;
impl<'a, REG> Ch6limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6limitlWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7limith {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch7limith> for bool {
    #[inline(always)]
    fn from(variant: Ch7limith) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITH` reader - Write '1' to enable interrupt for event CH7LIMITH"]
pub type Ch7limithR = crate::BitReader<Ch7limith>;
impl Ch7limithR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7limith {
        match self.bits {
            false => Ch7limith::Disabled,
            true => Ch7limith::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7limith::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7limith::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7limithWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch7limithWO> for bool {
    #[inline(always)]
    fn from(variant: Ch7limithWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITH` writer - Write '1' to enable interrupt for event CH7LIMITH"]
pub type Ch7limithW<'a, REG> = crate::BitWriter<'a, REG, Ch7limithWO>;
impl<'a, REG> Ch7limithW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7limithWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7limitl {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ch7limitl> for bool {
    #[inline(always)]
    fn from(variant: Ch7limitl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITL` reader - Write '1' to enable interrupt for event CH7LIMITL"]
pub type Ch7limitlR = crate::BitReader<Ch7limitl>;
impl Ch7limitlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7limitl {
        match self.bits {
            false => Ch7limitl::Disabled,
            true => Ch7limitl::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7limitl::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7limitl::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7limitlWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ch7limitlWO> for bool {
    #[inline(always)]
    fn from(variant: Ch7limitlWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITL` writer - Write '1' to enable interrupt for event CH7LIMITL"]
pub type Ch7limitlW<'a, REG> = crate::BitWriter<'a, REG, Ch7limitlWO>;
impl<'a, REG> Ch7limitlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7limitlWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> StartedR {
        StartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn resultdone(&self) -> ResultdoneR {
        ResultdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn calibratedone(&self) -> CalibratedoneR {
        CalibratedoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn ch0limith(&self) -> Ch0limithR {
        Ch0limithR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn ch0limitl(&self) -> Ch0limitlR {
        Ch0limitlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn ch1limith(&self) -> Ch1limithR {
        Ch1limithR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn ch1limitl(&self) -> Ch1limitlR {
        Ch1limitlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn ch2limith(&self) -> Ch2limithR {
        Ch2limithR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn ch2limitl(&self) -> Ch2limitlR {
        Ch2limitlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn ch3limith(&self) -> Ch3limithR {
        Ch3limithR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn ch3limitl(&self) -> Ch3limitlR {
        Ch3limitlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn ch4limith(&self) -> Ch4limithR {
        Ch4limithR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn ch4limitl(&self) -> Ch4limitlR {
        Ch4limitlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn ch5limith(&self) -> Ch5limithR {
        Ch5limithR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn ch5limitl(&self) -> Ch5limitlR {
        Ch5limitlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn ch6limith(&self) -> Ch6limithR {
        Ch6limithR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn ch6limitl(&self) -> Ch6limitlR {
        Ch6limitlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn ch7limith(&self) -> Ch7limithR {
        Ch7limithR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn ch7limitl(&self) -> Ch7limitlR {
        Ch7limitlR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> StartedW<'_, IntensetSpec> {
        StartedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<'_, IntensetSpec> {
        EndW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntensetSpec> {
        DoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn resultdone(&mut self) -> ResultdoneW<'_, IntensetSpec> {
        ResultdoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn calibratedone(&mut self) -> CalibratedoneW<'_, IntensetSpec> {
        CalibratedoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<'_, IntensetSpec> {
        StoppedW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn ch0limith(&mut self) -> Ch0limithW<'_, IntensetSpec> {
        Ch0limithW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn ch0limitl(&mut self) -> Ch0limitlW<'_, IntensetSpec> {
        Ch0limitlW::new(self, 7)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn ch1limith(&mut self) -> Ch1limithW<'_, IntensetSpec> {
        Ch1limithW::new(self, 8)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn ch1limitl(&mut self) -> Ch1limitlW<'_, IntensetSpec> {
        Ch1limitlW::new(self, 9)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn ch2limith(&mut self) -> Ch2limithW<'_, IntensetSpec> {
        Ch2limithW::new(self, 10)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn ch2limitl(&mut self) -> Ch2limitlW<'_, IntensetSpec> {
        Ch2limitlW::new(self, 11)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn ch3limith(&mut self) -> Ch3limithW<'_, IntensetSpec> {
        Ch3limithW::new(self, 12)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn ch3limitl(&mut self) -> Ch3limitlW<'_, IntensetSpec> {
        Ch3limitlW::new(self, 13)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn ch4limith(&mut self) -> Ch4limithW<'_, IntensetSpec> {
        Ch4limithW::new(self, 14)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn ch4limitl(&mut self) -> Ch4limitlW<'_, IntensetSpec> {
        Ch4limitlW::new(self, 15)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn ch5limith(&mut self) -> Ch5limithW<'_, IntensetSpec> {
        Ch5limithW::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn ch5limitl(&mut self) -> Ch5limitlW<'_, IntensetSpec> {
        Ch5limitlW::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn ch6limith(&mut self) -> Ch6limithW<'_, IntensetSpec> {
        Ch6limithW::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn ch6limitl(&mut self) -> Ch6limitlW<'_, IntensetSpec> {
        Ch6limitlW::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn ch7limith(&mut self) -> Ch7limithW<'_, IntensetSpec> {
        Ch7limithW::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn ch7limitl(&mut self) -> Ch7limitlW<'_, IntensetSpec> {
        Ch7limitlW::new(self, 21)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
