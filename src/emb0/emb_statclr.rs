#[doc = "Register `EMB_STATCLR` writer"]
pub type W = crate::W<EmbStatclrSpec>;
#[doc = "Field `PWMSFCLR` writer - desc PWMSFCLR"]
pub type PwmsfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFCLR` writer - desc CMPFCLR"]
pub type CmpfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFCLR` writer - desc SYSFCLR"]
pub type SysfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINFCLR1` writer - desc PORTINFCLR1"]
pub type Portinfclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINFCLR2` writer - desc PORTINFCLR2"]
pub type Portinfclr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINFCLR3` writer - desc PORTINFCLR3"]
pub type Portinfclr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINFCLR4` writer - desc PORTINFCLR4"]
pub type Portinfclr4W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EmbStatclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - desc PWMSFCLR"]
    #[inline(always)]
    pub fn pwmsfclr(&mut self) -> PwmsfclrW<'_, EmbStatclrSpec> {
        PwmsfclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPFCLR"]
    #[inline(always)]
    pub fn cmpfclr(&mut self) -> CmpfclrW<'_, EmbStatclrSpec> {
        CmpfclrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SYSFCLR"]
    #[inline(always)]
    pub fn sysfclr(&mut self) -> SysfclrW<'_, EmbStatclrSpec> {
        SysfclrW::new(self, 3)
    }
    #[doc = "Bit 8 - desc PORTINFCLR1"]
    #[inline(always)]
    pub fn portinfclr1(&mut self) -> Portinfclr1W<'_, EmbStatclrSpec> {
        Portinfclr1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc PORTINFCLR2"]
    #[inline(always)]
    pub fn portinfclr2(&mut self) -> Portinfclr2W<'_, EmbStatclrSpec> {
        Portinfclr2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc PORTINFCLR3"]
    #[inline(always)]
    pub fn portinfclr3(&mut self) -> Portinfclr3W<'_, EmbStatclrSpec> {
        Portinfclr3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc PORTINFCLR4"]
    #[inline(always)]
    pub fn portinfclr4(&mut self) -> Portinfclr4W<'_, EmbStatclrSpec> {
        Portinfclr4W::new(self, 11)
    }
}
#[doc = "desc EMB_STATCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_statclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmbStatclrSpec;
impl crate::RegisterSpec for EmbStatclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`emb_statclr::W`](W) writer structure"]
impl crate::Writable for EmbStatclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMB_STATCLR to value 0"]
impl crate::Resettable for EmbStatclrSpec {}
