#[doc = "Register `PLLHCR` reader"]
pub type R = crate::R<PllhcrSpec>;
#[doc = "Register `PLLHCR` writer"]
pub type W = crate::W<PllhcrSpec>;
#[doc = "Field `PLLHOFF` reader - desc PLLHOFF"]
pub type PllhoffR = crate::BitReader;
#[doc = "Field `PLLHOFF` writer - desc PLLHOFF"]
pub type PllhoffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PLLHOFF"]
    #[inline(always)]
    pub fn pllhoff(&self) -> PllhoffR {
        PllhoffR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLHCR")
            .field("pllhoff", &self.pllhoff())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc PLLHOFF"]
    #[inline(always)]
    pub fn pllhoff(&mut self) -> PllhoffW<'_, PllhcrSpec> {
        PllhoffW::new(self, 0)
    }
}
#[doc = "desc PLLHCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllhcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllhcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllhcrSpec;
impl crate::RegisterSpec for PllhcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllhcr::R`](R) reader structure"]
impl crate::Readable for PllhcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllhcr::W`](W) writer structure"]
impl crate::Writable for PllhcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLHCR to value 0x01"]
impl crate::Resettable for PllhcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
