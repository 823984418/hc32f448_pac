#[doc = "Register `TMR_SYNENR` reader"]
pub type R = crate::R<TmrSynenrSpec>;
#[doc = "Register `TMR_SYNENR` writer"]
pub type W = crate::W<TmrSynenrSpec>;
#[doc = "Field `TMR0U1A` reader - desc TMR0U1A"]
pub type Tmr0u1aR = crate::BitReader;
#[doc = "Field `TMR0U1A` writer - desc TMR0U1A"]
pub type Tmr0u1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0U1B` reader - desc TMR0U1B"]
pub type Tmr0u1bR = crate::BitReader;
#[doc = "Field `TMR0U1B` writer - desc TMR0U1B"]
pub type Tmr0u1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0U2A` reader - desc TMR0U2A"]
pub type Tmr0u2aR = crate::BitReader;
#[doc = "Field `TMR0U2A` writer - desc TMR0U2A"]
pub type Tmr0u2aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0U2B` reader - desc TMR0U2B"]
pub type Tmr0u2bR = crate::BitReader;
#[doc = "Field `TMR0U2B` writer - desc TMR0U2B"]
pub type Tmr0u2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4U1` reader - desc TMR4U1"]
pub type Tmr4u1R = crate::BitReader;
#[doc = "Field `TMR4U1` writer - desc TMR4U1"]
pub type Tmr4u1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4U2` reader - desc TMR4U2"]
pub type Tmr4u2R = crate::BitReader;
#[doc = "Field `TMR4U2` writer - desc TMR4U2"]
pub type Tmr4u2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4U3` reader - desc TMR4U3"]
pub type Tmr4u3R = crate::BitReader;
#[doc = "Field `TMR4U3` writer - desc TMR4U3"]
pub type Tmr4u3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6U1` reader - desc TMR6U1"]
pub type Tmr6u1R = crate::BitReader;
#[doc = "Field `TMR6U1` writer - desc TMR6U1"]
pub type Tmr6u1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6U2` reader - desc TMR6U2"]
pub type Tmr6u2R = crate::BitReader;
#[doc = "Field `TMR6U2` writer - desc TMR6U2"]
pub type Tmr6u2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRAU1` reader - desc TMRAU1"]
pub type Tmrau1R = crate::BitReader;
#[doc = "Field `TMRAU1` writer - desc TMRAU1"]
pub type Tmrau1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRAU2` reader - desc TMRAU2"]
pub type Tmrau2R = crate::BitReader;
#[doc = "Field `TMRAU2` writer - desc TMRAU2"]
pub type Tmrau2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRAU3` reader - desc TMRAU3"]
pub type Tmrau3R = crate::BitReader;
#[doc = "Field `TMRAU3` writer - desc TMRAU3"]
pub type Tmrau3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRAU4` reader - desc TMRAU4"]
pub type Tmrau4R = crate::BitReader;
#[doc = "Field `TMRAU4` writer - desc TMRAU4"]
pub type Tmrau4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRAU5` reader - desc TMRAU5"]
pub type Tmrau5R = crate::BitReader;
#[doc = "Field `TMRAU5` writer - desc TMRAU5"]
pub type Tmrau5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TMR0U1A"]
    #[inline(always)]
    pub fn tmr0u1a(&self) -> Tmr0u1aR {
        Tmr0u1aR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TMR0U1B"]
    #[inline(always)]
    pub fn tmr0u1b(&self) -> Tmr0u1bR {
        Tmr0u1bR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TMR0U2A"]
    #[inline(always)]
    pub fn tmr0u2a(&self) -> Tmr0u2aR {
        Tmr0u2aR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TMR0U2B"]
    #[inline(always)]
    pub fn tmr0u2b(&self) -> Tmr0u2bR {
        Tmr0u2bR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TMR4U1"]
    #[inline(always)]
    pub fn tmr4u1(&self) -> Tmr4u1R {
        Tmr4u1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TMR4U2"]
    #[inline(always)]
    pub fn tmr4u2(&self) -> Tmr4u2R {
        Tmr4u2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TMR4U3"]
    #[inline(always)]
    pub fn tmr4u3(&self) -> Tmr4u3R {
        Tmr4u3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TMR6U1"]
    #[inline(always)]
    pub fn tmr6u1(&self) -> Tmr6u1R {
        Tmr6u1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TMR6U2"]
    #[inline(always)]
    pub fn tmr6u2(&self) -> Tmr6u2R {
        Tmr6u2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TMRAU1"]
    #[inline(always)]
    pub fn tmrau1(&self) -> Tmrau1R {
        Tmrau1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TMRAU2"]
    #[inline(always)]
    pub fn tmrau2(&self) -> Tmrau2R {
        Tmrau2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TMRAU3"]
    #[inline(always)]
    pub fn tmrau3(&self) -> Tmrau3R {
        Tmrau3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TMRAU4"]
    #[inline(always)]
    pub fn tmrau4(&self) -> Tmrau4R {
        Tmrau4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TMRAU5"]
    #[inline(always)]
    pub fn tmrau5(&self) -> Tmrau5R {
        Tmrau5R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR_SYNENR")
            .field("tmr0u1a", &self.tmr0u1a())
            .field("tmr0u1b", &self.tmr0u1b())
            .field("tmr0u2a", &self.tmr0u2a())
            .field("tmr0u2b", &self.tmr0u2b())
            .field("tmr4u1", &self.tmr4u1())
            .field("tmr4u2", &self.tmr4u2())
            .field("tmr4u3", &self.tmr4u3())
            .field("tmr6u1", &self.tmr6u1())
            .field("tmr6u2", &self.tmr6u2())
            .field("tmrau1", &self.tmrau1())
            .field("tmrau2", &self.tmrau2())
            .field("tmrau3", &self.tmrau3())
            .field("tmrau4", &self.tmrau4())
            .field("tmrau5", &self.tmrau5())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc TMR0U1A"]
    #[inline(always)]
    pub fn tmr0u1a(&mut self) -> Tmr0u1aW<'_, TmrSynenrSpec> {
        Tmr0u1aW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TMR0U1B"]
    #[inline(always)]
    pub fn tmr0u1b(&mut self) -> Tmr0u1bW<'_, TmrSynenrSpec> {
        Tmr0u1bW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TMR0U2A"]
    #[inline(always)]
    pub fn tmr0u2a(&mut self) -> Tmr0u2aW<'_, TmrSynenrSpec> {
        Tmr0u2aW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TMR0U2B"]
    #[inline(always)]
    pub fn tmr0u2b(&mut self) -> Tmr0u2bW<'_, TmrSynenrSpec> {
        Tmr0u2bW::new(self, 3)
    }
    #[doc = "Bit 4 - desc TMR4U1"]
    #[inline(always)]
    pub fn tmr4u1(&mut self) -> Tmr4u1W<'_, TmrSynenrSpec> {
        Tmr4u1W::new(self, 4)
    }
    #[doc = "Bit 5 - desc TMR4U2"]
    #[inline(always)]
    pub fn tmr4u2(&mut self) -> Tmr4u2W<'_, TmrSynenrSpec> {
        Tmr4u2W::new(self, 5)
    }
    #[doc = "Bit 6 - desc TMR4U3"]
    #[inline(always)]
    pub fn tmr4u3(&mut self) -> Tmr4u3W<'_, TmrSynenrSpec> {
        Tmr4u3W::new(self, 6)
    }
    #[doc = "Bit 8 - desc TMR6U1"]
    #[inline(always)]
    pub fn tmr6u1(&mut self) -> Tmr6u1W<'_, TmrSynenrSpec> {
        Tmr6u1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc TMR6U2"]
    #[inline(always)]
    pub fn tmr6u2(&mut self) -> Tmr6u2W<'_, TmrSynenrSpec> {
        Tmr6u2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc TMRAU1"]
    #[inline(always)]
    pub fn tmrau1(&mut self) -> Tmrau1W<'_, TmrSynenrSpec> {
        Tmrau1W::new(self, 10)
    }
    #[doc = "Bit 11 - desc TMRAU2"]
    #[inline(always)]
    pub fn tmrau2(&mut self) -> Tmrau2W<'_, TmrSynenrSpec> {
        Tmrau2W::new(self, 11)
    }
    #[doc = "Bit 12 - desc TMRAU3"]
    #[inline(always)]
    pub fn tmrau3(&mut self) -> Tmrau3W<'_, TmrSynenrSpec> {
        Tmrau3W::new(self, 12)
    }
    #[doc = "Bit 13 - desc TMRAU4"]
    #[inline(always)]
    pub fn tmrau4(&mut self) -> Tmrau4W<'_, TmrSynenrSpec> {
        Tmrau4W::new(self, 13)
    }
    #[doc = "Bit 14 - desc TMRAU5"]
    #[inline(always)]
    pub fn tmrau5(&mut self) -> Tmrau5W<'_, TmrSynenrSpec> {
        Tmrau5W::new(self, 14)
    }
}
#[doc = "desc TMR_SYNENR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_synenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_synenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrSynenrSpec;
impl crate::RegisterSpec for TmrSynenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr_synenr::R`](R) reader structure"]
impl crate::Readable for TmrSynenrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr_synenr::W`](W) writer structure"]
impl crate::Writable for TmrSynenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR_SYNENR to value 0"]
impl crate::Resettable for TmrSynenrSpec {}
