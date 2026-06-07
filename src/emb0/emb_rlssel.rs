#[doc = "Register `EMB_RLSSEL` reader"]
pub type R = crate::R<EmbRlsselSpec>;
#[doc = "Register `EMB_RLSSEL` writer"]
pub type W = crate::W<EmbRlsselSpec>;
#[doc = "Field `PWMRSEL` reader - desc PWMRSEL"]
pub type PwmrselR = crate::BitReader;
#[doc = "Field `PWMRSEL` writer - desc PWMRSEL"]
pub type PwmrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRSEL` reader - desc CMPRSEL"]
pub type CmprselR = crate::BitReader;
#[doc = "Field `CMPRSEL` writer - desc CMPRSEL"]
pub type CmprselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSEL` reader - desc SYSRSEL"]
pub type SysrselR = crate::BitReader;
#[doc = "Field `SYSRSEL` writer - desc SYSRSEL"]
pub type SysrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINRSEL1` reader - desc PORTINRSEL1"]
pub type Portinrsel1R = crate::BitReader;
#[doc = "Field `PORTINRSEL1` writer - desc PORTINRSEL1"]
pub type Portinrsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINRSEL2` reader - desc PORTINRSEL2"]
pub type Portinrsel2R = crate::BitReader;
#[doc = "Field `PORTINRSEL2` writer - desc PORTINRSEL2"]
pub type Portinrsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINRSEL3` reader - desc PORTINRSEL3"]
pub type Portinrsel3R = crate::BitReader;
#[doc = "Field `PORTINRSEL3` writer - desc PORTINRSEL3"]
pub type Portinrsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTINRSEL4` reader - desc PORTINRSEL4"]
pub type Portinrsel4R = crate::BitReader;
#[doc = "Field `PORTINRSEL4` writer - desc PORTINRSEL4"]
pub type Portinrsel4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc PWMRSEL"]
    #[inline(always)]
    pub fn pwmrsel(&self) -> PwmrselR {
        PwmrselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPRSEL"]
    #[inline(always)]
    pub fn cmprsel(&self) -> CmprselR {
        CmprselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SYSRSEL"]
    #[inline(always)]
    pub fn sysrsel(&self) -> SysrselR {
        SysrselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PORTINRSEL1"]
    #[inline(always)]
    pub fn portinrsel1(&self) -> Portinrsel1R {
        Portinrsel1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PORTINRSEL2"]
    #[inline(always)]
    pub fn portinrsel2(&self) -> Portinrsel2R {
        Portinrsel2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PORTINRSEL3"]
    #[inline(always)]
    pub fn portinrsel3(&self) -> Portinrsel3R {
        Portinrsel3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PORTINRSEL4"]
    #[inline(always)]
    pub fn portinrsel4(&self) -> Portinrsel4R {
        Portinrsel4R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMB_RLSSEL")
            .field("pwmrsel", &self.pwmrsel())
            .field("cmprsel", &self.cmprsel())
            .field("sysrsel", &self.sysrsel())
            .field("portinrsel1", &self.portinrsel1())
            .field("portinrsel2", &self.portinrsel2())
            .field("portinrsel3", &self.portinrsel3())
            .field("portinrsel4", &self.portinrsel4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc PWMRSEL"]
    #[inline(always)]
    pub fn pwmrsel(&mut self) -> PwmrselW<'_, EmbRlsselSpec> {
        PwmrselW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPRSEL"]
    #[inline(always)]
    pub fn cmprsel(&mut self) -> CmprselW<'_, EmbRlsselSpec> {
        CmprselW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SYSRSEL"]
    #[inline(always)]
    pub fn sysrsel(&mut self) -> SysrselW<'_, EmbRlsselSpec> {
        SysrselW::new(self, 3)
    }
    #[doc = "Bit 8 - desc PORTINRSEL1"]
    #[inline(always)]
    pub fn portinrsel1(&mut self) -> Portinrsel1W<'_, EmbRlsselSpec> {
        Portinrsel1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc PORTINRSEL2"]
    #[inline(always)]
    pub fn portinrsel2(&mut self) -> Portinrsel2W<'_, EmbRlsselSpec> {
        Portinrsel2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc PORTINRSEL3"]
    #[inline(always)]
    pub fn portinrsel3(&mut self) -> Portinrsel3W<'_, EmbRlsselSpec> {
        Portinrsel3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc PORTINRSEL4"]
    #[inline(always)]
    pub fn portinrsel4(&mut self) -> Portinrsel4W<'_, EmbRlsselSpec> {
        Portinrsel4W::new(self, 11)
    }
}
#[doc = "desc EMB_RLSSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_rlssel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_rlssel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmbRlsselSpec;
impl crate::RegisterSpec for EmbRlsselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emb_rlssel::R`](R) reader structure"]
impl crate::Readable for EmbRlsselSpec {}
#[doc = "`write(|w| ..)` method takes [`emb_rlssel::W`](W) writer structure"]
impl crate::Writable for EmbRlsselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMB_RLSSEL to value 0"]
impl crate::Resettable for EmbRlsselSpec {}
