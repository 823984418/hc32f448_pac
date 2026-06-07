#[doc = "Register `FRST2` reader"]
pub type R = crate::R<Frst2Spec>;
#[doc = "Register `FRST2` writer"]
pub type W = crate::W<Frst2Spec>;
#[doc = "Field `TMR6` reader - desc TMR6"]
pub type Tmr6R = crate::BitReader;
#[doc = "Field `TMR6` writer - desc TMR6"]
pub type Tmr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4` reader - desc TMR4"]
pub type Tmr4R = crate::BitReader;
#[doc = "Field `TMR4` writer - desc TMR4"]
pub type Tmr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0` reader - desc TMR0"]
pub type Tmr0R = crate::BitReader;
#[doc = "Field `TMR0` writer - desc TMR0"]
pub type Tmr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMB` reader - desc EMB"]
pub type EmbR = crate::BitReader;
#[doc = "Field `EMB` writer - desc EMB"]
pub type EmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA` reader - desc TMRA"]
pub type TmraR = crate::BitReader;
#[doc = "Field `TMRA` writer - desc TMRA"]
pub type TmraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TMR6"]
    #[inline(always)]
    pub fn tmr6(&self) -> Tmr6R {
        Tmr6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - desc TMR4"]
    #[inline(always)]
    pub fn tmr4(&self) -> Tmr4R {
        Tmr4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TMR0"]
    #[inline(always)]
    pub fn tmr0(&self) -> Tmr0R {
        Tmr0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&self) -> EmbR {
        EmbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TMRA"]
    #[inline(always)]
    pub fn tmra(&self) -> TmraR {
        TmraR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRST2")
            .field("tmr6", &self.tmr6())
            .field("tmr4", &self.tmr4())
            .field("tmr0", &self.tmr0())
            .field("emb", &self.emb())
            .field("tmra", &self.tmra())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc TMR6"]
    #[inline(always)]
    pub fn tmr6(&mut self) -> Tmr6W<'_, Frst2Spec> {
        Tmr6W::new(self, 0)
    }
    #[doc = "Bit 10 - desc TMR4"]
    #[inline(always)]
    pub fn tmr4(&mut self) -> Tmr4W<'_, Frst2Spec> {
        Tmr4W::new(self, 10)
    }
    #[doc = "Bit 12 - desc TMR0"]
    #[inline(always)]
    pub fn tmr0(&mut self) -> Tmr0W<'_, Frst2Spec> {
        Tmr0W::new(self, 12)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&mut self) -> EmbW<'_, Frst2Spec> {
        EmbW::new(self, 15)
    }
    #[doc = "Bit 20 - desc TMRA"]
    #[inline(always)]
    pub fn tmra(&mut self) -> TmraW<'_, Frst2Spec> {
        TmraW::new(self, 20)
    }
}
#[doc = "desc FRST2\n\nYou can [`read`](crate::Reg::read) this register and get [`frst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frst2Spec;
impl crate::RegisterSpec for Frst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frst2::R`](R) reader structure"]
impl crate::Readable for Frst2Spec {}
#[doc = "`write(|w| ..)` method takes [`frst2::W`](W) writer structure"]
impl crate::Writable for Frst2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRST2 to value 0xffff_ffff"]
impl crate::Resettable for Frst2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
