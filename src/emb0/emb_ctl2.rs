#[doc = "Register `EMB_CTL2` reader"]
pub type R = crate::R<EmbCtl2Spec>;
#[doc = "Register `EMB_CTL2` writer"]
pub type W = crate::W<EmbCtl2Spec>;
#[doc = "Field `PWMLV0` reader - desc PWMLV0"]
pub type Pwmlv0R = crate::BitReader;
#[doc = "Field `PWMLV0` writer - desc PWMLV0"]
pub type Pwmlv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMLV1` reader - desc PWMLV1"]
pub type Pwmlv1R = crate::BitReader;
#[doc = "Field `PWMLV1` writer - desc PWMLV1"]
pub type Pwmlv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFSEL1` reader - desc NFSEL1"]
pub type Nfsel1R = crate::FieldReader;
#[doc = "Field `NFSEL1` writer - desc NFSEL1"]
pub type Nfsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN1` reader - desc NFEN1"]
pub type Nfen1R = crate::BitReader;
#[doc = "Field `NFEN1` writer - desc NFEN1"]
pub type Nfen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFSEL2` reader - desc NFSEL2"]
pub type Nfsel2R = crate::FieldReader;
#[doc = "Field `NFSEL2` writer - desc NFSEL2"]
pub type Nfsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN2` reader - desc NFEN2"]
pub type Nfen2R = crate::BitReader;
#[doc = "Field `NFEN2` writer - desc NFEN2"]
pub type Nfen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFSEL3` reader - desc NFSEL3"]
pub type Nfsel3R = crate::FieldReader;
#[doc = "Field `NFSEL3` writer - desc NFSEL3"]
pub type Nfsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN3` reader - desc NFEN3"]
pub type Nfen3R = crate::BitReader;
#[doc = "Field `NFEN3` writer - desc NFEN3"]
pub type Nfen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFSEL4` reader - desc NFSEL4"]
pub type Nfsel4R = crate::FieldReader;
#[doc = "Field `NFSEL4` writer - desc NFSEL4"]
pub type Nfsel4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN4` reader - desc NFEN4"]
pub type Nfen4R = crate::BitReader;
#[doc = "Field `NFEN4` writer - desc NFEN4"]
pub type Nfen4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PWMLV0"]
    #[inline(always)]
    pub fn pwmlv0(&self) -> Pwmlv0R {
        Pwmlv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PWMLV1"]
    #[inline(always)]
    pub fn pwmlv1(&self) -> Pwmlv1R {
        Pwmlv1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc NFSEL1"]
    #[inline(always)]
    pub fn nfsel1(&self) -> Nfsel1R {
        Nfsel1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - desc NFEN1"]
    #[inline(always)]
    pub fn nfen1(&self) -> Nfen1R {
        Nfen1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - desc NFSEL2"]
    #[inline(always)]
    pub fn nfsel2(&self) -> Nfsel2R {
        Nfsel2R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - desc NFEN2"]
    #[inline(always)]
    pub fn nfen2(&self) -> Nfen2R {
        Nfen2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - desc NFSEL3"]
    #[inline(always)]
    pub fn nfsel3(&self) -> Nfsel3R {
        Nfsel3R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - desc NFEN3"]
    #[inline(always)]
    pub fn nfen3(&self) -> Nfen3R {
        Nfen3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc NFSEL4"]
    #[inline(always)]
    pub fn nfsel4(&self) -> Nfsel4R {
        Nfsel4R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - desc NFEN4"]
    #[inline(always)]
    pub fn nfen4(&self) -> Nfen4R {
        Nfen4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMB_CTL2")
            .field("pwmlv0", &self.pwmlv0())
            .field("pwmlv1", &self.pwmlv1())
            .field("nfsel1", &self.nfsel1())
            .field("nfen1", &self.nfen1())
            .field("nfsel2", &self.nfsel2())
            .field("nfen2", &self.nfen2())
            .field("nfsel3", &self.nfsel3())
            .field("nfen3", &self.nfen3())
            .field("nfsel4", &self.nfsel4())
            .field("nfen4", &self.nfen4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc PWMLV0"]
    #[inline(always)]
    pub fn pwmlv0(&mut self) -> Pwmlv0W<'_, EmbCtl2Spec> {
        Pwmlv0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PWMLV1"]
    #[inline(always)]
    pub fn pwmlv1(&mut self) -> Pwmlv1W<'_, EmbCtl2Spec> {
        Pwmlv1W::new(self, 1)
    }
    #[doc = "Bits 16:17 - desc NFSEL1"]
    #[inline(always)]
    pub fn nfsel1(&mut self) -> Nfsel1W<'_, EmbCtl2Spec> {
        Nfsel1W::new(self, 16)
    }
    #[doc = "Bit 18 - desc NFEN1"]
    #[inline(always)]
    pub fn nfen1(&mut self) -> Nfen1W<'_, EmbCtl2Spec> {
        Nfen1W::new(self, 18)
    }
    #[doc = "Bits 19:20 - desc NFSEL2"]
    #[inline(always)]
    pub fn nfsel2(&mut self) -> Nfsel2W<'_, EmbCtl2Spec> {
        Nfsel2W::new(self, 19)
    }
    #[doc = "Bit 21 - desc NFEN2"]
    #[inline(always)]
    pub fn nfen2(&mut self) -> Nfen2W<'_, EmbCtl2Spec> {
        Nfen2W::new(self, 21)
    }
    #[doc = "Bits 22:23 - desc NFSEL3"]
    #[inline(always)]
    pub fn nfsel3(&mut self) -> Nfsel3W<'_, EmbCtl2Spec> {
        Nfsel3W::new(self, 22)
    }
    #[doc = "Bit 24 - desc NFEN3"]
    #[inline(always)]
    pub fn nfen3(&mut self) -> Nfen3W<'_, EmbCtl2Spec> {
        Nfen3W::new(self, 24)
    }
    #[doc = "Bits 25:26 - desc NFSEL4"]
    #[inline(always)]
    pub fn nfsel4(&mut self) -> Nfsel4W<'_, EmbCtl2Spec> {
        Nfsel4W::new(self, 25)
    }
    #[doc = "Bit 27 - desc NFEN4"]
    #[inline(always)]
    pub fn nfen4(&mut self) -> Nfen4W<'_, EmbCtl2Spec> {
        Nfen4W::new(self, 27)
    }
}
#[doc = "desc EMB_CTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmbCtl2Spec;
impl crate::RegisterSpec for EmbCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emb_ctl2::R`](R) reader structure"]
impl crate::Readable for EmbCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`emb_ctl2::W`](W) writer structure"]
impl crate::Writable for EmbCtl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMB_CTL2 to value 0"]
impl crate::Resettable for EmbCtl2Spec {}
