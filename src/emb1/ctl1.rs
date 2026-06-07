#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `CMPEN1` reader - desc CMPEN1"]
pub type Cmpen1R = crate::BitReader;
#[doc = "Field `CMPEN1` writer - desc CMPEN1"]
pub type Cmpen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN2` reader - desc CMPEN2"]
pub type Cmpen2R = crate::BitReader;
#[doc = "Field `CMPEN2` writer - desc CMPEN2"]
pub type Cmpen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN3` reader - desc CMPEN3"]
pub type Cmpen3R = crate::BitReader;
#[doc = "Field `CMPEN3` writer - desc CMPEN3"]
pub type Cmpen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN4` reader - desc CMPEN4"]
pub type Cmpen4R = crate::BitReader;
#[doc = "Field `CMPEN4` writer - desc CMPEN4"]
pub type Cmpen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSEN` reader - desc SYSEN"]
pub type SysenR = crate::BitReader;
#[doc = "Field `SYSEN` writer - desc SYSEN"]
pub type SysenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN0` reader - desc PWMSEN0"]
pub type Pwmsen0R = crate::BitReader;
#[doc = "Field `PWMSEN0` writer - desc PWMSEN0"]
pub type Pwmsen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN1` reader - desc PWMSEN1"]
pub type Pwmsen1R = crate::BitReader;
#[doc = "Field `PWMSEN1` writer - desc PWMSEN1"]
pub type Pwmsen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN2` reader - desc PWMSEN2"]
pub type Pwmsen2R = crate::BitReader;
#[doc = "Field `PWMSEN2` writer - desc PWMSEN2"]
pub type Pwmsen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN3` reader - desc PWMSEN3"]
pub type Pwmsen3R = crate::BitReader;
#[doc = "Field `PWMSEN3` writer - desc PWMSEN3"]
pub type Pwmsen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINEN1` reader - desc PORTINEN1"]
pub type Portinen1R = crate::BitReader;
#[doc = "Field `PORTINEN1` writer - desc PORTINEN1"]
pub type Portinen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINEN2` reader - desc PORTINEN2"]
pub type Portinen2R = crate::BitReader;
#[doc = "Field `PORTINEN2` writer - desc PORTINEN2"]
pub type Portinen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINEN3` reader - desc PORTINEN3"]
pub type Portinen3R = crate::BitReader;
#[doc = "Field `PORTINEN3` writer - desc PORTINEN3"]
pub type Portinen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINEN4` reader - desc PORTINEN4"]
pub type Portinen4R = crate::BitReader;
#[doc = "Field `PORTINEN4` writer - desc PORTINEN4"]
pub type Portinen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSEL1` reader - desc INVSEL1"]
pub type Invsel1R = crate::BitReader;
#[doc = "Field `INVSEL1` writer - desc INVSEL1"]
pub type Invsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSEL2` reader - desc INVSEL2"]
pub type Invsel2R = crate::BitReader;
#[doc = "Field `INVSEL2` writer - desc INVSEL2"]
pub type Invsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSEL3` reader - desc INVSEL3"]
pub type Invsel3R = crate::BitReader;
#[doc = "Field `INVSEL3` writer - desc INVSEL3"]
pub type Invsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSEL4` reader - desc INVSEL4"]
pub type Invsel4R = crate::BitReader;
#[doc = "Field `INVSEL4` writer - desc INVSEL4"]
pub type Invsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCSTPEN` reader - desc OSCSTPEN"]
pub type OscstpenR = crate::BitReader;
#[doc = "Field `OSCSTPEN` writer - desc OSCSTPEN"]
pub type OscstpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMECCERREN` reader - desc SRAMECCERREN"]
pub type SrameccerrenR = crate::BitReader;
#[doc = "Field `SRAMECCERREN` writer - desc SRAMECCERREN"]
pub type SrameccerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMPYERREN` reader - desc SRAMPYERREN"]
pub type SrampyerrenR = crate::BitReader;
#[doc = "Field `SRAMPYERREN` writer - desc SRAMPYERREN"]
pub type SrampyerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUPEN` reader - desc LOCKUPEN"]
pub type LockupenR = crate::BitReader;
#[doc = "Field `LOCKUPEN` writer - desc LOCKUPEN"]
pub type LockupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDEN` reader - desc PVDEN"]
pub type PvdenR = crate::BitReader;
#[doc = "Field `PVDEN` writer - desc PVDEN"]
pub type PvdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMPEN1"]
    #[inline(always)]
    pub fn cmpen1(&self) -> Cmpen1R {
        Cmpen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMPEN2"]
    #[inline(always)]
    pub fn cmpen2(&self) -> Cmpen2R {
        Cmpen2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPEN3"]
    #[inline(always)]
    pub fn cmpen3(&self) -> Cmpen3R {
        Cmpen3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPEN4"]
    #[inline(always)]
    pub fn cmpen4(&self) -> Cmpen4R {
        Cmpen4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SYSEN"]
    #[inline(always)]
    pub fn sysen(&self) -> SysenR {
        SysenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PWMSEN0"]
    #[inline(always)]
    pub fn pwmsen0(&self) -> Pwmsen0R {
        Pwmsen0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PWMSEN1"]
    #[inline(always)]
    pub fn pwmsen1(&self) -> Pwmsen1R {
        Pwmsen1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PWMSEN2"]
    #[inline(always)]
    pub fn pwmsen2(&self) -> Pwmsen2R {
        Pwmsen2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PWMSEN3"]
    #[inline(always)]
    pub fn pwmsen3(&self) -> Pwmsen3R {
        Pwmsen3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc PORTINEN1"]
    #[inline(always)]
    pub fn portinen1(&self) -> Portinen1R {
        Portinen1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc PORTINEN2"]
    #[inline(always)]
    pub fn portinen2(&self) -> Portinen2R {
        Portinen2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc PORTINEN3"]
    #[inline(always)]
    pub fn portinen3(&self) -> Portinen3R {
        Portinen3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc PORTINEN4"]
    #[inline(always)]
    pub fn portinen4(&self) -> Portinen4R {
        Portinen4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - desc INVSEL1"]
    #[inline(always)]
    pub fn invsel1(&self) -> Invsel1R {
        Invsel1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc INVSEL2"]
    #[inline(always)]
    pub fn invsel2(&self) -> Invsel2R {
        Invsel2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc INVSEL3"]
    #[inline(always)]
    pub fn invsel3(&self) -> Invsel3R {
        Invsel3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc INVSEL4"]
    #[inline(always)]
    pub fn invsel4(&self) -> Invsel4R {
        Invsel4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - desc OSCSTPEN"]
    #[inline(always)]
    pub fn oscstpen(&self) -> OscstpenR {
        OscstpenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc SRAMECCERREN"]
    #[inline(always)]
    pub fn srameccerren(&self) -> SrameccerrenR {
        SrameccerrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc SRAMPYERREN"]
    #[inline(always)]
    pub fn srampyerren(&self) -> SrampyerrenR {
        SrampyerrenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc LOCKUPEN"]
    #[inline(always)]
    pub fn lockupen(&self) -> LockupenR {
        LockupenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc PVDEN"]
    #[inline(always)]
    pub fn pvden(&self) -> PvdenR {
        PvdenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("cmpen1", &self.cmpen1())
            .field("cmpen2", &self.cmpen2())
            .field("cmpen3", &self.cmpen3())
            .field("cmpen4", &self.cmpen4())
            .field("sysen", &self.sysen())
            .field("pwmsen0", &self.pwmsen0())
            .field("pwmsen1", &self.pwmsen1())
            .field("pwmsen2", &self.pwmsen2())
            .field("pwmsen3", &self.pwmsen3())
            .field("portinen1", &self.portinen1())
            .field("portinen2", &self.portinen2())
            .field("portinen3", &self.portinen3())
            .field("portinen4", &self.portinen4())
            .field("invsel1", &self.invsel1())
            .field("invsel2", &self.invsel2())
            .field("invsel3", &self.invsel3())
            .field("invsel4", &self.invsel4())
            .field("oscstpen", &self.oscstpen())
            .field("srameccerren", &self.srameccerren())
            .field("srampyerren", &self.srampyerren())
            .field("lockupen", &self.lockupen())
            .field("pvden", &self.pvden())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CMPEN1"]
    #[inline(always)]
    pub fn cmpen1(&mut self) -> Cmpen1W<'_, Ctl1Spec> {
        Cmpen1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc CMPEN2"]
    #[inline(always)]
    pub fn cmpen2(&mut self) -> Cmpen2W<'_, Ctl1Spec> {
        Cmpen2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPEN3"]
    #[inline(always)]
    pub fn cmpen3(&mut self) -> Cmpen3W<'_, Ctl1Spec> {
        Cmpen3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMPEN4"]
    #[inline(always)]
    pub fn cmpen4(&mut self) -> Cmpen4W<'_, Ctl1Spec> {
        Cmpen4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc SYSEN"]
    #[inline(always)]
    pub fn sysen(&mut self) -> SysenW<'_, Ctl1Spec> {
        SysenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc PWMSEN0"]
    #[inline(always)]
    pub fn pwmsen0(&mut self) -> Pwmsen0W<'_, Ctl1Spec> {
        Pwmsen0W::new(self, 5)
    }
    #[doc = "Bit 6 - desc PWMSEN1"]
    #[inline(always)]
    pub fn pwmsen1(&mut self) -> Pwmsen1W<'_, Ctl1Spec> {
        Pwmsen1W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PWMSEN2"]
    #[inline(always)]
    pub fn pwmsen2(&mut self) -> Pwmsen2W<'_, Ctl1Spec> {
        Pwmsen2W::new(self, 7)
    }
    #[doc = "Bit 8 - desc PWMSEN3"]
    #[inline(always)]
    pub fn pwmsen3(&mut self) -> Pwmsen3W<'_, Ctl1Spec> {
        Pwmsen3W::new(self, 8)
    }
    #[doc = "Bit 16 - desc PORTINEN1"]
    #[inline(always)]
    pub fn portinen1(&mut self) -> Portinen1W<'_, Ctl1Spec> {
        Portinen1W::new(self, 16)
    }
    #[doc = "Bit 17 - desc PORTINEN2"]
    #[inline(always)]
    pub fn portinen2(&mut self) -> Portinen2W<'_, Ctl1Spec> {
        Portinen2W::new(self, 17)
    }
    #[doc = "Bit 18 - desc PORTINEN3"]
    #[inline(always)]
    pub fn portinen3(&mut self) -> Portinen3W<'_, Ctl1Spec> {
        Portinen3W::new(self, 18)
    }
    #[doc = "Bit 19 - desc PORTINEN4"]
    #[inline(always)]
    pub fn portinen4(&mut self) -> Portinen4W<'_, Ctl1Spec> {
        Portinen4W::new(self, 19)
    }
    #[doc = "Bit 22 - desc INVSEL1"]
    #[inline(always)]
    pub fn invsel1(&mut self) -> Invsel1W<'_, Ctl1Spec> {
        Invsel1W::new(self, 22)
    }
    #[doc = "Bit 23 - desc INVSEL2"]
    #[inline(always)]
    pub fn invsel2(&mut self) -> Invsel2W<'_, Ctl1Spec> {
        Invsel2W::new(self, 23)
    }
    #[doc = "Bit 24 - desc INVSEL3"]
    #[inline(always)]
    pub fn invsel3(&mut self) -> Invsel3W<'_, Ctl1Spec> {
        Invsel3W::new(self, 24)
    }
    #[doc = "Bit 25 - desc INVSEL4"]
    #[inline(always)]
    pub fn invsel4(&mut self) -> Invsel4W<'_, Ctl1Spec> {
        Invsel4W::new(self, 25)
    }
    #[doc = "Bit 27 - desc OSCSTPEN"]
    #[inline(always)]
    pub fn oscstpen(&mut self) -> OscstpenW<'_, Ctl1Spec> {
        OscstpenW::new(self, 27)
    }
    #[doc = "Bit 28 - desc SRAMECCERREN"]
    #[inline(always)]
    pub fn srameccerren(&mut self) -> SrameccerrenW<'_, Ctl1Spec> {
        SrameccerrenW::new(self, 28)
    }
    #[doc = "Bit 29 - desc SRAMPYERREN"]
    #[inline(always)]
    pub fn srampyerren(&mut self) -> SrampyerrenW<'_, Ctl1Spec> {
        SrampyerrenW::new(self, 29)
    }
    #[doc = "Bit 30 - desc LOCKUPEN"]
    #[inline(always)]
    pub fn lockupen(&mut self) -> LockupenW<'_, Ctl1Spec> {
        LockupenW::new(self, 30)
    }
    #[doc = "Bit 31 - desc PVDEN"]
    #[inline(always)]
    pub fn pvden(&mut self) -> PvdenW<'_, Ctl1Spec> {
        PvdenW::new(self, 31)
    }
}
#[doc = "desc CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
