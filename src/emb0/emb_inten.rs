#[doc = "Register `EMB_INTEN` reader"]
pub type R = crate::R<EmbIntenSpec>;
#[doc = "Register `EMB_INTEN` writer"]
pub type W = crate::W<EmbIntenSpec>;
#[doc = "Field `PWMSINTEN` reader - desc PWMSINTEN"]
pub type PwmsintenR = crate::BitReader;
#[doc = "Field `PWMSINTEN` writer - desc PWMSINTEN"]
pub type PwmsintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPINTEN` reader - desc CMPINTEN"]
pub type CmpintenR = crate::BitReader;
#[doc = "Field `CMPINTEN` writer - desc CMPINTEN"]
pub type CmpintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSINTEN` reader - desc SYSINTEN"]
pub type SysintenR = crate::BitReader;
#[doc = "Field `SYSINTEN` writer - desc SYSINTEN"]
pub type SysintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTININTEN1` reader - desc PORTININTEN1"]
pub type Portininten1R = crate::BitReader;
#[doc = "Field `PORTININTEN1` writer - desc PORTININTEN1"]
pub type Portininten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTININTEN2` reader - desc PORTININTEN2"]
pub type Portininten2R = crate::BitReader;
#[doc = "Field `PORTININTEN2` writer - desc PORTININTEN2"]
pub type Portininten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTININTEN3` reader - desc PORTININTEN3"]
pub type Portininten3R = crate::BitReader;
#[doc = "Field `PORTININTEN3` writer - desc PORTININTEN3"]
pub type Portininten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTININTEN4` reader - desc PORTININTEN4"]
pub type Portininten4R = crate::BitReader;
#[doc = "Field `PORTININTEN4` writer - desc PORTININTEN4"]
pub type Portininten4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc PWMSINTEN"]
    #[inline(always)]
    pub fn pwmsinten(&self) -> PwmsintenR {
        PwmsintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPINTEN"]
    #[inline(always)]
    pub fn cmpinten(&self) -> CmpintenR {
        CmpintenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SYSINTEN"]
    #[inline(always)]
    pub fn sysinten(&self) -> SysintenR {
        SysintenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PORTININTEN1"]
    #[inline(always)]
    pub fn portininten1(&self) -> Portininten1R {
        Portininten1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PORTININTEN2"]
    #[inline(always)]
    pub fn portininten2(&self) -> Portininten2R {
        Portininten2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PORTININTEN3"]
    #[inline(always)]
    pub fn portininten3(&self) -> Portininten3R {
        Portininten3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PORTININTEN4"]
    #[inline(always)]
    pub fn portininten4(&self) -> Portininten4R {
        Portininten4R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMB_INTEN")
            .field("pwmsinten", &self.pwmsinten())
            .field("cmpinten", &self.cmpinten())
            .field("sysinten", &self.sysinten())
            .field("portininten1", &self.portininten1())
            .field("portininten2", &self.portininten2())
            .field("portininten3", &self.portininten3())
            .field("portininten4", &self.portininten4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc PWMSINTEN"]
    #[inline(always)]
    pub fn pwmsinten(&mut self) -> PwmsintenW<'_, EmbIntenSpec> {
        PwmsintenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPINTEN"]
    #[inline(always)]
    pub fn cmpinten(&mut self) -> CmpintenW<'_, EmbIntenSpec> {
        CmpintenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SYSINTEN"]
    #[inline(always)]
    pub fn sysinten(&mut self) -> SysintenW<'_, EmbIntenSpec> {
        SysintenW::new(self, 3)
    }
    #[doc = "Bit 8 - desc PORTININTEN1"]
    #[inline(always)]
    pub fn portininten1(&mut self) -> Portininten1W<'_, EmbIntenSpec> {
        Portininten1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc PORTININTEN2"]
    #[inline(always)]
    pub fn portininten2(&mut self) -> Portininten2W<'_, EmbIntenSpec> {
        Portininten2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc PORTININTEN3"]
    #[inline(always)]
    pub fn portininten3(&mut self) -> Portininten3W<'_, EmbIntenSpec> {
        Portininten3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc PORTININTEN4"]
    #[inline(always)]
    pub fn portininten4(&mut self) -> Portininten4W<'_, EmbIntenSpec> {
        Portininten4W::new(self, 11)
    }
}
#[doc = "desc EMB_INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmbIntenSpec;
impl crate::RegisterSpec for EmbIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emb_inten::R`](R) reader structure"]
impl crate::Readable for EmbIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`emb_inten::W`](W) writer structure"]
impl crate::Writable for EmbIntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMB_INTEN to value 0"]
impl crate::Resettable for EmbIntenSpec {}
