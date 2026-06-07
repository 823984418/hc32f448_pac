#[doc = "Register `PLLHCFGR` reader"]
pub type R = crate::R<PllhcfgrSpec>;
#[doc = "Register `PLLHCFGR` writer"]
pub type W = crate::W<PllhcfgrSpec>;
#[doc = "Field `PLLHM` reader - desc PLLHM"]
pub type PllhmR = crate::FieldReader;
#[doc = "Field `PLLHM` writer - desc PLLHM"]
pub type PllhmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLSRC` reader - desc PLLSRC"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - desc PLLSRC"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLHN` reader - desc PLLHN"]
pub type PllhnR = crate::FieldReader;
#[doc = "Field `PLLHN` writer - desc PLLHN"]
pub type PllhnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLHR` reader - desc PLLHR"]
pub type PllhrR = crate::FieldReader;
#[doc = "Field `PLLHR` writer - desc PLLHR"]
pub type PllhrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLHQ` reader - desc PLLHQ"]
pub type PllhqR = crate::FieldReader;
#[doc = "Field `PLLHQ` writer - desc PLLHQ"]
pub type PllhqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLHP` reader - desc PLLHP"]
pub type PllhpR = crate::FieldReader;
#[doc = "Field `PLLHP` writer - desc PLLHP"]
pub type PllhpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - desc PLLHM"]
    #[inline(always)]
    pub fn pllhm(&self) -> PllhmR {
        PllhmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc PLLHN"]
    #[inline(always)]
    pub fn pllhn(&self) -> PllhnR {
        PllhnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - desc PLLHR"]
    #[inline(always)]
    pub fn pllhr(&self) -> PllhrR {
        PllhrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc PLLHQ"]
    #[inline(always)]
    pub fn pllhq(&self) -> PllhqR {
        PllhqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc PLLHP"]
    #[inline(always)]
    pub fn pllhp(&self) -> PllhpR {
        PllhpR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLHCFGR")
            .field("pllhm", &self.pllhm())
            .field("pllsrc", &self.pllsrc())
            .field("pllhn", &self.pllhn())
            .field("pllhr", &self.pllhr())
            .field("pllhq", &self.pllhq())
            .field("pllhp", &self.pllhp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PLLHM"]
    #[inline(always)]
    pub fn pllhm(&mut self) -> PllhmW<'_, PllhcfgrSpec> {
        PllhmW::new(self, 0)
    }
    #[doc = "Bit 7 - desc PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, PllhcfgrSpec> {
        PllsrcW::new(self, 7)
    }
    #[doc = "Bits 8:15 - desc PLLHN"]
    #[inline(always)]
    pub fn pllhn(&mut self) -> PllhnW<'_, PllhcfgrSpec> {
        PllhnW::new(self, 8)
    }
    #[doc = "Bits 20:23 - desc PLLHR"]
    #[inline(always)]
    pub fn pllhr(&mut self) -> PllhrW<'_, PllhcfgrSpec> {
        PllhrW::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc PLLHQ"]
    #[inline(always)]
    pub fn pllhq(&mut self) -> PllhqW<'_, PllhcfgrSpec> {
        PllhqW::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc PLLHP"]
    #[inline(always)]
    pub fn pllhp(&mut self) -> PllhpW<'_, PllhcfgrSpec> {
        PllhpW::new(self, 28)
    }
}
#[doc = "desc PLLHCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllhcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllhcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllhcfgrSpec;
impl crate::RegisterSpec for PllhcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllhcfgr::R`](R) reader structure"]
impl crate::Readable for PllhcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllhcfgr::W`](W) writer structure"]
impl crate::Writable for PllhcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLHCFGR to value 0x1110_1300"]
impl crate::Resettable for PllhcfgrSpec {
    const RESET_VALUE: u32 = 0x1110_1300;
}
