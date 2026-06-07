#[doc = "Register `FCG2` reader"]
pub type R = crate::R<Fcg2Spec>;
#[doc = "Register `FCG2` writer"]
pub type W = crate::W<Fcg2Spec>;
#[doc = "Field `TMR6_1` reader - desc TMR6_1"]
pub type Tmr6_1R = crate::BitReader;
#[doc = "Field `TMR6_1` writer - desc TMR6_1"]
pub type Tmr6_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_2` reader - desc TMR6_2"]
pub type Tmr6_2R = crate::BitReader;
#[doc = "Field `TMR6_2` writer - desc TMR6_2"]
pub type Tmr6_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_1` reader - desc TMR4_1"]
pub type Tmr4_1R = crate::BitReader;
#[doc = "Field `TMR4_1` writer - desc TMR4_1"]
pub type Tmr4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_2` reader - desc TMR4_2"]
pub type Tmr4_2R = crate::BitReader;
#[doc = "Field `TMR4_2` writer - desc TMR4_2"]
pub type Tmr4_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_3` reader - desc TMR4_3"]
pub type Tmr4_3R = crate::BitReader;
#[doc = "Field `TMR4_3` writer - desc TMR4_3"]
pub type Tmr4_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0_1` reader - desc TMR0_1"]
pub type Tmr0_1R = crate::BitReader;
#[doc = "Field `TMR0_1` writer - desc TMR0_1"]
pub type Tmr0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0_2` reader - desc TMR0_2"]
pub type Tmr0_2R = crate::BitReader;
#[doc = "Field `TMR0_2` writer - desc TMR0_2"]
pub type Tmr0_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMB` reader - desc EMB"]
pub type EmbR = crate::BitReader;
#[doc = "Field `EMB` writer - desc EMB"]
pub type EmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA_1` reader - desc TMRA_1"]
pub type Tmra1R = crate::BitReader;
#[doc = "Field `TMRA_1` writer - desc TMRA_1"]
pub type Tmra1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA_2` reader - desc TMRA_2"]
pub type Tmra2R = crate::BitReader;
#[doc = "Field `TMRA_2` writer - desc TMRA_2"]
pub type Tmra2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA_3` reader - desc TMRA_3"]
pub type Tmra3R = crate::BitReader;
#[doc = "Field `TMRA_3` writer - desc TMRA_3"]
pub type Tmra3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA_4` reader - desc TMRA_4"]
pub type Tmra4R = crate::BitReader;
#[doc = "Field `TMRA_4` writer - desc TMRA_4"]
pub type Tmra4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRA_5` reader - desc TMRA_5"]
pub type Tmra5R = crate::BitReader;
#[doc = "Field `TMRA_5` writer - desc TMRA_5"]
pub type Tmra5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TMR6_1"]
    #[inline(always)]
    pub fn tmr6_1(&self) -> Tmr6_1R {
        Tmr6_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TMR6_2"]
    #[inline(always)]
    pub fn tmr6_2(&self) -> Tmr6_2R {
        Tmr6_2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - desc TMR4_1"]
    #[inline(always)]
    pub fn tmr4_1(&self) -> Tmr4_1R {
        Tmr4_1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TMR4_2"]
    #[inline(always)]
    pub fn tmr4_2(&self) -> Tmr4_2R {
        Tmr4_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TMR4_3"]
    #[inline(always)]
    pub fn tmr4_3(&self) -> Tmr4_3R {
        Tmr4_3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TMR0_1"]
    #[inline(always)]
    pub fn tmr0_1(&self) -> Tmr0_1R {
        Tmr0_1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TMR0_2"]
    #[inline(always)]
    pub fn tmr0_2(&self) -> Tmr0_2R {
        Tmr0_2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&self) -> EmbR {
        EmbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - desc TMRA_1"]
    #[inline(always)]
    pub fn tmra_1(&self) -> Tmra1R {
        Tmra1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TMRA_2"]
    #[inline(always)]
    pub fn tmra_2(&self) -> Tmra2R {
        Tmra2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc TMRA_3"]
    #[inline(always)]
    pub fn tmra_3(&self) -> Tmra3R {
        Tmra3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TMRA_4"]
    #[inline(always)]
    pub fn tmra_4(&self) -> Tmra4R {
        Tmra4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TMRA_5"]
    #[inline(always)]
    pub fn tmra_5(&self) -> Tmra5R {
        Tmra5R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCG2")
            .field("tmr6_1", &self.tmr6_1())
            .field("tmr6_2", &self.tmr6_2())
            .field("tmr4_1", &self.tmr4_1())
            .field("tmr4_2", &self.tmr4_2())
            .field("tmr4_3", &self.tmr4_3())
            .field("tmr0_1", &self.tmr0_1())
            .field("tmr0_2", &self.tmr0_2())
            .field("emb", &self.emb())
            .field("tmra_1", &self.tmra_1())
            .field("tmra_2", &self.tmra_2())
            .field("tmra_3", &self.tmra_3())
            .field("tmra_4", &self.tmra_4())
            .field("tmra_5", &self.tmra_5())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc TMR6_1"]
    #[inline(always)]
    pub fn tmr6_1(&mut self) -> Tmr6_1W<'_, Fcg2Spec> {
        Tmr6_1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc TMR6_2"]
    #[inline(always)]
    pub fn tmr6_2(&mut self) -> Tmr6_2W<'_, Fcg2Spec> {
        Tmr6_2W::new(self, 1)
    }
    #[doc = "Bit 9 - desc TMR4_1"]
    #[inline(always)]
    pub fn tmr4_1(&mut self) -> Tmr4_1W<'_, Fcg2Spec> {
        Tmr4_1W::new(self, 9)
    }
    #[doc = "Bit 10 - desc TMR4_2"]
    #[inline(always)]
    pub fn tmr4_2(&mut self) -> Tmr4_2W<'_, Fcg2Spec> {
        Tmr4_2W::new(self, 10)
    }
    #[doc = "Bit 11 - desc TMR4_3"]
    #[inline(always)]
    pub fn tmr4_3(&mut self) -> Tmr4_3W<'_, Fcg2Spec> {
        Tmr4_3W::new(self, 11)
    }
    #[doc = "Bit 12 - desc TMR0_1"]
    #[inline(always)]
    pub fn tmr0_1(&mut self) -> Tmr0_1W<'_, Fcg2Spec> {
        Tmr0_1W::new(self, 12)
    }
    #[doc = "Bit 13 - desc TMR0_2"]
    #[inline(always)]
    pub fn tmr0_2(&mut self) -> Tmr0_2W<'_, Fcg2Spec> {
        Tmr0_2W::new(self, 13)
    }
    #[doc = "Bit 15 - desc EMB"]
    #[inline(always)]
    pub fn emb(&mut self) -> EmbW<'_, Fcg2Spec> {
        EmbW::new(self, 15)
    }
    #[doc = "Bit 20 - desc TMRA_1"]
    #[inline(always)]
    pub fn tmra_1(&mut self) -> Tmra1W<'_, Fcg2Spec> {
        Tmra1W::new(self, 20)
    }
    #[doc = "Bit 21 - desc TMRA_2"]
    #[inline(always)]
    pub fn tmra_2(&mut self) -> Tmra2W<'_, Fcg2Spec> {
        Tmra2W::new(self, 21)
    }
    #[doc = "Bit 22 - desc TMRA_3"]
    #[inline(always)]
    pub fn tmra_3(&mut self) -> Tmra3W<'_, Fcg2Spec> {
        Tmra3W::new(self, 22)
    }
    #[doc = "Bit 23 - desc TMRA_4"]
    #[inline(always)]
    pub fn tmra_4(&mut self) -> Tmra4W<'_, Fcg2Spec> {
        Tmra4W::new(self, 23)
    }
    #[doc = "Bit 24 - desc TMRA_5"]
    #[inline(always)]
    pub fn tmra_5(&mut self) -> Tmra5W<'_, Fcg2Spec> {
        Tmra5W::new(self, 24)
    }
}
#[doc = "desc FCG2\n\nYou can [`read`](crate::Reg::read) this register and get [`fcg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcg2Spec;
impl crate::RegisterSpec for Fcg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcg2::R`](R) reader structure"]
impl crate::Readable for Fcg2Spec {}
#[doc = "`write(|w| ..)` method takes [`fcg2::W`](W) writer structure"]
impl crate::Writable for Fcg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCG2 to value 0xffff_ffff"]
impl crate::Resettable for Fcg2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
