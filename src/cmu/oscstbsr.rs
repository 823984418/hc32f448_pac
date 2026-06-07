#[doc = "Register `OSCSTBSR` reader"]
pub type R = crate::R<OscstbsrSpec>;
#[doc = "Register `OSCSTBSR` writer"]
pub type W = crate::W<OscstbsrSpec>;
#[doc = "Field `HRCSTBF` reader - desc HRCSTBF"]
pub type HrcstbfR = crate::BitReader;
#[doc = "Field `HRCSTBF` writer - desc HRCSTBF"]
pub type HrcstbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTBF` reader - desc XTALSTBF"]
pub type XtalstbfR = crate::BitReader;
#[doc = "Field `XTALSTBF` writer - desc XTALSTBF"]
pub type XtalstbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLHSTBF` reader - desc PLLHSTBF"]
pub type PllhstbfR = crate::BitReader;
#[doc = "Field `PLLHSTBF` writer - desc PLLHSTBF"]
pub type PllhstbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HRCSTBF"]
    #[inline(always)]
    pub fn hrcstbf(&self) -> HrcstbfR {
        HrcstbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTALSTBF"]
    #[inline(always)]
    pub fn xtalstbf(&self) -> XtalstbfR {
        XtalstbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PLLHSTBF"]
    #[inline(always)]
    pub fn pllhstbf(&self) -> PllhstbfR {
        PllhstbfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSCSTBSR")
            .field("hrcstbf", &self.hrcstbf())
            .field("xtalstbf", &self.xtalstbf())
            .field("pllhstbf", &self.pllhstbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc HRCSTBF"]
    #[inline(always)]
    pub fn hrcstbf(&mut self) -> HrcstbfW<'_, OscstbsrSpec> {
        HrcstbfW::new(self, 0)
    }
    #[doc = "Bit 3 - desc XTALSTBF"]
    #[inline(always)]
    pub fn xtalstbf(&mut self) -> XtalstbfW<'_, OscstbsrSpec> {
        XtalstbfW::new(self, 3)
    }
    #[doc = "Bit 5 - desc PLLHSTBF"]
    #[inline(always)]
    pub fn pllhstbf(&mut self) -> PllhstbfW<'_, OscstbsrSpec> {
        PllhstbfW::new(self, 5)
    }
}
#[doc = "desc OSCSTBSR\n\nYou can [`read`](crate::Reg::read) this register and get [`oscstbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscstbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscstbsrSpec;
impl crate::RegisterSpec for OscstbsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oscstbsr::R`](R) reader structure"]
impl crate::Readable for OscstbsrSpec {}
#[doc = "`write(|w| ..)` method takes [`oscstbsr::W`](W) writer structure"]
impl crate::Writable for OscstbsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSCSTBSR to value 0"]
impl crate::Resettable for OscstbsrSpec {}
