#[doc = "Register `PSCR` reader"]
pub type R = crate::R<PscrSpec>;
#[doc = "Register `PSCR` writer"]
pub type W = crate::W<PscrSpec>;
#[doc = "Field `OEUH` reader - desc OEUH"]
pub type OeuhR = crate::BitReader;
#[doc = "Field `OEUH` writer - desc OEUH"]
pub type OeuhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEUL` reader - desc OEUL"]
pub type OeulR = crate::BitReader;
#[doc = "Field `OEUL` writer - desc OEUL"]
pub type OeulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEVH` reader - desc OEVH"]
pub type OevhR = crate::BitReader;
#[doc = "Field `OEVH` writer - desc OEVH"]
pub type OevhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEVL` reader - desc OEVL"]
pub type OevlR = crate::BitReader;
#[doc = "Field `OEVL` writer - desc OEVL"]
pub type OevlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEWH` reader - desc OEWH"]
pub type OewhR = crate::BitReader;
#[doc = "Field `OEWH` writer - desc OEWH"]
pub type OewhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEWL` reader - desc OEWL"]
pub type OewlR = crate::BitReader;
#[doc = "Field `OEWL` writer - desc OEWL"]
pub type OewlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEXH` reader - desc OEXH"]
pub type OexhR = crate::BitReader;
#[doc = "Field `OEXH` writer - desc OEXH"]
pub type OexhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEXL` reader - desc OEXL"]
pub type OexlR = crate::BitReader;
#[doc = "Field `OEXL` writer - desc OEXL"]
pub type OexlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - desc MOE"]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - desc AOE"]
pub type AoeR = crate::BitReader;
#[doc = "Field `AOE` writer - desc AOE"]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT` reader - desc ODT"]
pub type OdtR = crate::FieldReader;
#[doc = "Field `ODT` writer - desc ODT"]
pub type OdtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSUH` reader - desc OSUH"]
pub type OsuhR = crate::FieldReader;
#[doc = "Field `OSUH` writer - desc OSUH"]
pub type OsuhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSUL` reader - desc OSUL"]
pub type OsulR = crate::FieldReader;
#[doc = "Field `OSUL` writer - desc OSUL"]
pub type OsulW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSVH` reader - desc OSVH"]
pub type OsvhR = crate::FieldReader;
#[doc = "Field `OSVH` writer - desc OSVH"]
pub type OsvhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSVL` reader - desc OSVL"]
pub type OsvlR = crate::FieldReader;
#[doc = "Field `OSVL` writer - desc OSVL"]
pub type OsvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSWH` reader - desc OSWH"]
pub type OswhR = crate::FieldReader;
#[doc = "Field `OSWH` writer - desc OSWH"]
pub type OswhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSWL` reader - desc OSWL"]
pub type OswlR = crate::FieldReader;
#[doc = "Field `OSWL` writer - desc OSWL"]
pub type OswlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSXH` reader - desc OSXH"]
pub type OsxhR = crate::FieldReader;
#[doc = "Field `OSXH` writer - desc OSXH"]
pub type OsxhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSXL` reader - desc OSXL"]
pub type OsxlR = crate::FieldReader;
#[doc = "Field `OSXL` writer - desc OSXL"]
pub type OsxlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc OEUH"]
    #[inline(always)]
    pub fn oeuh(&self) -> OeuhR {
        OeuhR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OEUL"]
    #[inline(always)]
    pub fn oeul(&self) -> OeulR {
        OeulR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OEVH"]
    #[inline(always)]
    pub fn oevh(&self) -> OevhR {
        OevhR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OEVL"]
    #[inline(always)]
    pub fn oevl(&self) -> OevlR {
        OevlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OEWH"]
    #[inline(always)]
    pub fn oewh(&self) -> OewhR {
        OewhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OEWL"]
    #[inline(always)]
    pub fn oewl(&self) -> OewlR {
        OewlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OEXH"]
    #[inline(always)]
    pub fn oexh(&self) -> OexhR {
        OexhR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc OEXL"]
    #[inline(always)]
    pub fn oexl(&self) -> OexlR {
        OexlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - desc ODT"]
    #[inline(always)]
    pub fn odt(&self) -> OdtR {
        OdtR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc OSUH"]
    #[inline(always)]
    pub fn osuh(&self) -> OsuhR {
        OsuhR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - desc OSUL"]
    #[inline(always)]
    pub fn osul(&self) -> OsulR {
        OsulR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc OSVH"]
    #[inline(always)]
    pub fn osvh(&self) -> OsvhR {
        OsvhR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc OSVL"]
    #[inline(always)]
    pub fn osvl(&self) -> OsvlR {
        OsvlR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - desc OSWH"]
    #[inline(always)]
    pub fn oswh(&self) -> OswhR {
        OswhR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - desc OSWL"]
    #[inline(always)]
    pub fn oswl(&self) -> OswlR {
        OswlR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - desc OSXH"]
    #[inline(always)]
    pub fn osxh(&self) -> OsxhR {
        OsxhR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - desc OSXL"]
    #[inline(always)]
    pub fn osxl(&self) -> OsxlR {
        OsxlR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSCR")
            .field("oeuh", &self.oeuh())
            .field("oeul", &self.oeul())
            .field("oevh", &self.oevh())
            .field("oevl", &self.oevl())
            .field("oewh", &self.oewh())
            .field("oewl", &self.oewl())
            .field("oexh", &self.oexh())
            .field("oexl", &self.oexl())
            .field("moe", &self.moe())
            .field("aoe", &self.aoe())
            .field("odt", &self.odt())
            .field("osuh", &self.osuh())
            .field("osul", &self.osul())
            .field("osvh", &self.osvh())
            .field("osvl", &self.osvl())
            .field("oswh", &self.oswh())
            .field("oswl", &self.oswl())
            .field("osxh", &self.osxh())
            .field("osxl", &self.osxl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc OEUH"]
    #[inline(always)]
    pub fn oeuh(&mut self) -> OeuhW<'_, PscrSpec> {
        OeuhW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OEUL"]
    #[inline(always)]
    pub fn oeul(&mut self) -> OeulW<'_, PscrSpec> {
        OeulW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OEVH"]
    #[inline(always)]
    pub fn oevh(&mut self) -> OevhW<'_, PscrSpec> {
        OevhW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OEVL"]
    #[inline(always)]
    pub fn oevl(&mut self) -> OevlW<'_, PscrSpec> {
        OevlW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OEWH"]
    #[inline(always)]
    pub fn oewh(&mut self) -> OewhW<'_, PscrSpec> {
        OewhW::new(self, 4)
    }
    #[doc = "Bit 5 - desc OEWL"]
    #[inline(always)]
    pub fn oewl(&mut self) -> OewlW<'_, PscrSpec> {
        OewlW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OEXH"]
    #[inline(always)]
    pub fn oexh(&mut self) -> OexhW<'_, PscrSpec> {
        OexhW::new(self, 6)
    }
    #[doc = "Bit 7 - desc OEXL"]
    #[inline(always)]
    pub fn oexl(&mut self) -> OexlW<'_, PscrSpec> {
        OexlW::new(self, 7)
    }
    #[doc = "Bit 8 - desc MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<'_, PscrSpec> {
        MoeW::new(self, 8)
    }
    #[doc = "Bit 9 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<'_, PscrSpec> {
        AoeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - desc ODT"]
    #[inline(always)]
    pub fn odt(&mut self) -> OdtW<'_, PscrSpec> {
        OdtW::new(self, 10)
    }
    #[doc = "Bits 16:17 - desc OSUH"]
    #[inline(always)]
    pub fn osuh(&mut self) -> OsuhW<'_, PscrSpec> {
        OsuhW::new(self, 16)
    }
    #[doc = "Bits 18:19 - desc OSUL"]
    #[inline(always)]
    pub fn osul(&mut self) -> OsulW<'_, PscrSpec> {
        OsulW::new(self, 18)
    }
    #[doc = "Bits 20:21 - desc OSVH"]
    #[inline(always)]
    pub fn osvh(&mut self) -> OsvhW<'_, PscrSpec> {
        OsvhW::new(self, 20)
    }
    #[doc = "Bits 22:23 - desc OSVL"]
    #[inline(always)]
    pub fn osvl(&mut self) -> OsvlW<'_, PscrSpec> {
        OsvlW::new(self, 22)
    }
    #[doc = "Bits 24:25 - desc OSWH"]
    #[inline(always)]
    pub fn oswh(&mut self) -> OswhW<'_, PscrSpec> {
        OswhW::new(self, 24)
    }
    #[doc = "Bits 26:27 - desc OSWL"]
    #[inline(always)]
    pub fn oswl(&mut self) -> OswlW<'_, PscrSpec> {
        OswlW::new(self, 26)
    }
    #[doc = "Bits 28:29 - desc OSXH"]
    #[inline(always)]
    pub fn osxh(&mut self) -> OsxhW<'_, PscrSpec> {
        OsxhW::new(self, 28)
    }
    #[doc = "Bits 30:31 - desc OSXL"]
    #[inline(always)]
    pub fn osxl(&mut self) -> OsxlW<'_, PscrSpec> {
        OsxlW::new(self, 30)
    }
}
#[doc = "desc PSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscrSpec;
impl crate::RegisterSpec for PscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pscr::R`](R) reader structure"]
impl crate::Readable for PscrSpec {}
#[doc = "`write(|w| ..)` method takes [`pscr::W`](W) writer structure"]
impl crate::Writable for PscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSCR to value 0x5555_0000"]
impl crate::Resettable for PscrSpec {
    const RESET_VALUE: u32 = 0x5555_0000;
}
