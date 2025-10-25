#[doc = "Register `FFCR` reader"]
pub type R = crate::R<FfcrSpec>;
#[doc = "Register `FFCR` writer"]
pub type W = crate::W<FfcrSpec>;
#[doc = "Do not embed triggers into the formatted stream. Trace disable cycles and triggers are indicated by tracectl, where present.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enftc {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Enftc> for bool {
    #[inline(always)]
    fn from(variant: Enftc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENFTC` reader - Do not embed triggers into the formatted stream. Trace disable cycles and triggers are indicated by tracectl, where present."]
pub type EnftcR = crate::BitReader<Enftc>;
impl EnftcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enftc {
        match self.bits {
            false => Enftc::Disabled,
            true => Enftc::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enftc::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enftc::Enabled
    }
}
#[doc = "Field `ENFTC` writer - Do not embed triggers into the formatted stream. Trace disable cycles and triggers are indicated by tracectl, where present."]
pub type EnftcW<'a, REG> = crate::BitWriter<'a, REG, Enftc>;
impl<'a, REG> EnftcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enftc::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enftc::Enabled)
    }
}
#[doc = "Is embedded in trigger packets and indicates that no cycle is using sync packets.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enfcont {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Enfcont> for bool {
    #[inline(always)]
    fn from(variant: Enfcont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENFCONT` reader - Is embedded in trigger packets and indicates that no cycle is using sync packets."]
pub type EnfcontR = crate::BitReader<Enfcont>;
impl EnfcontR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enfcont {
        match self.bits {
            false => Enfcont::Disabled,
            true => Enfcont::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enfcont::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enfcont::Enabled
    }
}
#[doc = "Field `ENFCONT` writer - Is embedded in trigger packets and indicates that no cycle is using sync packets."]
pub type EnfcontW<'a, REG> = crate::BitWriter<'a, REG, Enfcont>;
impl<'a, REG> EnfcontW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enfcont::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enfcont::Enabled)
    }
}
#[doc = "Enables the use of the flushin connection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fonflin {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Fonflin> for bool {
    #[inline(always)]
    fn from(variant: Fonflin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FONFLIN` reader - Enables the use of the flushin connection."]
pub type FonflinR = crate::BitReader<Fonflin>;
impl FonflinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fonflin {
        match self.bits {
            false => Fonflin::Disabled,
            true => Fonflin::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fonflin::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fonflin::Enabled
    }
}
#[doc = "Field `FONFLIN` writer - Enables the use of the flushin connection."]
pub type FonflinW<'a, REG> = crate::BitWriter<'a, REG, Fonflin>;
impl<'a, REG> FonflinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonflin::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonflin::Enabled)
    }
}
#[doc = "Initiates a manual flush of data in the system when a trigger event occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fontrig {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Fontrig> for bool {
    #[inline(always)]
    fn from(variant: Fontrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FONTRIG` reader - Initiates a manual flush of data in the system when a trigger event occurs."]
pub type FontrigR = crate::BitReader<Fontrig>;
impl FontrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fontrig {
        match self.bits {
            false => Fontrig::Disabled,
            true => Fontrig::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fontrig::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fontrig::Enabled
    }
}
#[doc = "Field `FONTRIG` writer - Initiates a manual flush of data in the system when a trigger event occurs."]
pub type FontrigW<'a, REG> = crate::BitWriter<'a, REG, Fontrig>;
impl<'a, REG> FontrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fontrig::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fontrig::Enabled)
    }
}
#[doc = "Generates a flush. This bit is set to 0 when this flush is serviced.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fonmanr {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Fonmanr> for bool {
    #[inline(always)]
    fn from(variant: Fonmanr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FONMANR` reader - Generates a flush. This bit is set to 0 when this flush is serviced."]
pub type FonmanrR = crate::BitReader<Fonmanr>;
impl FonmanrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fonmanr {
        match self.bits {
            false => Fonmanr::Disabled,
            true => Fonmanr::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fonmanr::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fonmanr::Enabled
    }
}
#[doc = "Field `FONMANR` writer - Generates a flush. This bit is set to 0 when this flush is serviced."]
pub type FonmanrW<'a, REG> = crate::BitWriter<'a, REG, Fonmanr>;
impl<'a, REG> FonmanrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonmanr::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonmanr::Enabled)
    }
}
#[doc = "Generates a flush. This bit is set to 1 when this flush is serviced.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fonmanw {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Fonmanw> for bool {
    #[inline(always)]
    fn from(variant: Fonmanw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FONMANW` reader - Generates a flush. This bit is set to 1 when this flush is serviced."]
pub type FonmanwR = crate::BitReader<Fonmanw>;
impl FonmanwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fonmanw {
        match self.bits {
            false => Fonmanw::Disabled,
            true => Fonmanw::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fonmanw::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fonmanw::Enabled
    }
}
#[doc = "Field `FONMANW` writer - Generates a flush. This bit is set to 1 when this flush is serviced."]
pub type FonmanwW<'a, REG> = crate::BitWriter<'a, REG, Fonmanw>;
impl<'a, REG> FonmanwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonmanw::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fonmanw::Enabled)
    }
}
#[doc = "Indicates a trigger when trigin is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigin {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Trigin> for bool {
    #[inline(always)]
    fn from(variant: Trigin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGIN` reader - Indicates a trigger when trigin is asserted."]
pub type TriginR = crate::BitReader<Trigin>;
impl TriginR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigin {
        match self.bits {
            false => Trigin::Disabled,
            true => Trigin::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigin::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigin::Enabled
    }
}
#[doc = "Field `TRIGIN` writer - Indicates a trigger when trigin is asserted."]
pub type TriginW<'a, REG> = crate::BitWriter<'a, REG, Trigin>;
impl<'a, REG> TriginW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigin::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigin::Enabled)
    }
}
#[doc = "Indicates a trigger on a trigger event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigevt {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Trigevt> for bool {
    #[inline(always)]
    fn from(variant: Trigevt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGEVT` reader - Indicates a trigger on a trigger event."]
pub type TrigevtR = crate::BitReader<Trigevt>;
impl TrigevtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigevt {
        match self.bits {
            false => Trigevt::Disabled,
            true => Trigevt::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigevt::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigevt::Enabled
    }
}
#[doc = "Field `TRIGEVT` writer - Indicates a trigger on a trigger event."]
pub type TrigevtW<'a, REG> = crate::BitWriter<'a, REG, Trigevt>;
impl<'a, REG> TrigevtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigevt::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigevt::Enabled)
    }
}
#[doc = "Indicates a trigger when flush completion on afreadys is returned.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigfl {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Trigfl> for bool {
    #[inline(always)]
    fn from(variant: Trigfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGFL` reader - Indicates a trigger when flush completion on afreadys is returned."]
pub type TrigflR = crate::BitReader<Trigfl>;
impl TrigflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigfl {
        match self.bits {
            false => Trigfl::Disabled,
            true => Trigfl::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trigfl::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trigfl::Enabled
    }
}
#[doc = "Field `TRIGFL` writer - Indicates a trigger when flush completion on afreadys is returned."]
pub type TrigflW<'a, REG> = crate::BitWriter<'a, REG, Trigfl>;
impl<'a, REG> TrigflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigfl::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Trigfl::Enabled)
    }
}
#[doc = "Forces the FIFO to drain off any part-completed packets.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopfl {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Stopfl> for bool {
    #[inline(always)]
    fn from(variant: Stopfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPFL` reader - Forces the FIFO to drain off any part-completed packets."]
pub type StopflR = crate::BitReader<Stopfl>;
impl StopflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopfl {
        match self.bits {
            false => Stopfl::Disabled,
            true => Stopfl::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopfl::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopfl::Enabled
    }
}
#[doc = "Field `STOPFL` writer - Forces the FIFO to drain off any part-completed packets."]
pub type StopflW<'a, REG> = crate::BitWriter<'a, REG, Stopfl>;
impl<'a, REG> StopflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopfl::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopfl::Enabled)
    }
}
#[doc = "Stops the formatter after a trigger event is observed. Reset to disabled or 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stoptrig {
    #[doc = "0: The formatting feature is disabled."]
    Disabled = 0,
    #[doc = "1: The formatting feature is enabled."]
    Enabled = 1,
}
impl From<Stoptrig> for bool {
    #[inline(always)]
    fn from(variant: Stoptrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPTRIG` reader - Stops the formatter after a trigger event is observed. Reset to disabled or 0."]
pub type StoptrigR = crate::BitReader<Stoptrig>;
impl StoptrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stoptrig {
        match self.bits {
            false => Stoptrig::Disabled,
            true => Stoptrig::Enabled,
        }
    }
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stoptrig::Disabled
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stoptrig::Enabled
    }
}
#[doc = "Field `STOPTRIG` writer - Stops the formatter after a trigger event is observed. Reset to disabled or 0."]
pub type StoptrigW<'a, REG> = crate::BitWriter<'a, REG, Stoptrig>;
impl<'a, REG> StoptrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The formatting feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stoptrig::Disabled)
    }
    #[doc = "The formatting feature is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stoptrig::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Do not embed triggers into the formatted stream. Trace disable cycles and triggers are indicated by tracectl, where present."]
    #[inline(always)]
    pub fn enftc(&self) -> EnftcR {
        EnftcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Is embedded in trigger packets and indicates that no cycle is using sync packets."]
    #[inline(always)]
    pub fn enfcont(&self) -> EnfcontR {
        EnfcontR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the use of the flushin connection."]
    #[inline(always)]
    pub fn fonflin(&self) -> FonflinR {
        FonflinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Initiates a manual flush of data in the system when a trigger event occurs."]
    #[inline(always)]
    pub fn fontrig(&self) -> FontrigR {
        FontrigR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generates a flush. This bit is set to 0 when this flush is serviced."]
    #[inline(always)]
    pub fn fonmanr(&self) -> FonmanrR {
        FonmanrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generates a flush. This bit is set to 1 when this flush is serviced."]
    #[inline(always)]
    pub fn fonmanw(&self) -> FonmanwR {
        FonmanwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates a trigger when trigin is asserted."]
    #[inline(always)]
    pub fn trigin(&self) -> TriginR {
        TriginR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates a trigger on a trigger event."]
    #[inline(always)]
    pub fn trigevt(&self) -> TrigevtR {
        TrigevtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates a trigger when flush completion on afreadys is returned."]
    #[inline(always)]
    pub fn trigfl(&self) -> TrigflR {
        TrigflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Forces the FIFO to drain off any part-completed packets."]
    #[inline(always)]
    pub fn stopfl(&self) -> StopflR {
        StopflR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stops the formatter after a trigger event is observed. Reset to disabled or 0."]
    #[inline(always)]
    pub fn stoptrig(&self) -> StoptrigR {
        StoptrigR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Do not embed triggers into the formatted stream. Trace disable cycles and triggers are indicated by tracectl, where present."]
    #[inline(always)]
    pub fn enftc(&mut self) -> EnftcW<'_, FfcrSpec> {
        EnftcW::new(self, 0)
    }
    #[doc = "Bit 1 - Is embedded in trigger packets and indicates that no cycle is using sync packets."]
    #[inline(always)]
    pub fn enfcont(&mut self) -> EnfcontW<'_, FfcrSpec> {
        EnfcontW::new(self, 1)
    }
    #[doc = "Bit 4 - Enables the use of the flushin connection."]
    #[inline(always)]
    pub fn fonflin(&mut self) -> FonflinW<'_, FfcrSpec> {
        FonflinW::new(self, 4)
    }
    #[doc = "Bit 5 - Initiates a manual flush of data in the system when a trigger event occurs."]
    #[inline(always)]
    pub fn fontrig(&mut self) -> FontrigW<'_, FfcrSpec> {
        FontrigW::new(self, 5)
    }
    #[doc = "Bit 6 - Generates a flush. This bit is set to 0 when this flush is serviced."]
    #[inline(always)]
    pub fn fonmanr(&mut self) -> FonmanrW<'_, FfcrSpec> {
        FonmanrW::new(self, 6)
    }
    #[doc = "Bit 7 - Generates a flush. This bit is set to 1 when this flush is serviced."]
    #[inline(always)]
    pub fn fonmanw(&mut self) -> FonmanwW<'_, FfcrSpec> {
        FonmanwW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates a trigger when trigin is asserted."]
    #[inline(always)]
    pub fn trigin(&mut self) -> TriginW<'_, FfcrSpec> {
        TriginW::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates a trigger on a trigger event."]
    #[inline(always)]
    pub fn trigevt(&mut self) -> TrigevtW<'_, FfcrSpec> {
        TrigevtW::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates a trigger when flush completion on afreadys is returned."]
    #[inline(always)]
    pub fn trigfl(&mut self) -> TrigflW<'_, FfcrSpec> {
        TrigflW::new(self, 10)
    }
    #[doc = "Bit 12 - Forces the FIFO to drain off any part-completed packets."]
    #[inline(always)]
    pub fn stopfl(&mut self) -> StopflW<'_, FfcrSpec> {
        StopflW::new(self, 12)
    }
    #[doc = "Bit 13 - Stops the formatter after a trigger event is observed. Reset to disabled or 0."]
    #[inline(always)]
    pub fn stoptrig(&mut self) -> StoptrigW<'_, FfcrSpec> {
        StoptrigW::new(self, 13)
    }
}
#[doc = "The FFCR register controls the generation of stop, trigger, and flush events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FfcrSpec;
impl crate::RegisterSpec for FfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffcr::R`](R) reader structure"]
impl crate::Readable for FfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ffcr::W`](W) writer structure"]
impl crate::Writable for FfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FfcrSpec {}
