#[doc = "Register `FCNGR` reader"]
pub type R = crate::R<FcngrSpec>;
#[doc = "Register `FCNGR` writer"]
pub type W = crate::W<FcngrSpec>;
#[doc = "Field `NOFIENGA` reader - desc NOFIENGA"]
pub type NofiengaR = crate::BitReader;
#[doc = "Field `NOFIENGA` writer - desc NOFIENGA"]
pub type NofiengaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKGA` reader - desc NOFICKGA"]
pub type NofickgaR = crate::FieldReader;
#[doc = "Field `NOFICKGA` writer - desc NOFICKGA"]
pub type NofickgaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENGB` reader - desc NOFIENGB"]
pub type NofiengbR = crate::BitReader;
#[doc = "Field `NOFIENGB` writer - desc NOFIENGB"]
pub type NofiengbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKGB` reader - desc NOFICKGB"]
pub type NofickgbR = crate::FieldReader;
#[doc = "Field `NOFICKGB` writer - desc NOFICKGB"]
pub type NofickgbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&self) -> NofiengaR {
        NofiengaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&self) -> NofickgaR {
        NofickgaR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&self) -> NofiengbR {
        NofiengbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&self) -> NofickgbR {
        NofickgbR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCNGR")
            .field("nofienga", &self.nofienga())
            .field("nofickga", &self.nofickga())
            .field("nofiengb", &self.nofiengb())
            .field("nofickgb", &self.nofickgb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&mut self) -> NofiengaW<'_, FcngrSpec> {
        NofiengaW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&mut self) -> NofickgaW<'_, FcngrSpec> {
        NofickgaW::new(self, 1)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&mut self) -> NofiengbW<'_, FcngrSpec> {
        NofiengbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&mut self) -> NofickgbW<'_, FcngrSpec> {
        NofickgbW::new(self, 5)
    }
}
#[doc = "desc FCNGR\n\nYou can [`read`](crate::Reg::read) this register and get [`fcngr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcngr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcngrSpec;
impl crate::RegisterSpec for FcngrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcngr::R`](R) reader structure"]
impl crate::Readable for FcngrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcngr::W`](W) writer structure"]
impl crate::Writable for FcngrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCNGR to value 0"]
impl crate::Resettable for FcngrSpec {}
