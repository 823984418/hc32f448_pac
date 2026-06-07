#[doc = "Register `FRMC` reader"]
pub type R = crate::R<FrmcSpec>;
#[doc = "Register `FRMC` writer"]
pub type W = crate::W<FrmcSpec>;
#[doc = "Field `FLWT` reader - desc FLWT"]
pub type FlwtR = crate::FieldReader;
#[doc = "Field `FLWT` writer - desc FLWT"]
pub type FlwtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LVM` reader - desc LVM"]
pub type LvmR = crate::BitReader;
#[doc = "Field `LVM` writer - desc LVM"]
pub type LvmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE` reader - desc ICACHE"]
pub type IcacheR = crate::BitReader;
#[doc = "Field `ICACHE` writer - desc ICACHE"]
pub type IcacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE` reader - desc DCACHE"]
pub type DcacheR = crate::BitReader;
#[doc = "Field `DCACHE` writer - desc DCACHE"]
pub type DcacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETE` reader - desc PREFETE"]
pub type PrefeteR = crate::BitReader;
#[doc = "Field `PREFETE` writer - desc PREFETE"]
pub type PrefeteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRST` reader - desc CRST"]
pub type CrstR = crate::BitReader;
#[doc = "Field `CRST` writer - desc CRST"]
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc FLWT"]
    #[inline(always)]
    pub fn flwt(&self) -> FlwtR {
        FlwtR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc LVM"]
    #[inline(always)]
    pub fn lvm(&self) -> LvmR {
        LvmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc ICACHE"]
    #[inline(always)]
    pub fn icache(&self) -> IcacheR {
        IcacheR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DCACHE"]
    #[inline(always)]
    pub fn dcache(&self) -> DcacheR {
        DcacheR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc PREFETE"]
    #[inline(always)]
    pub fn prefete(&self) -> PrefeteR {
        PrefeteR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CRST"]
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRMC")
            .field("flwt", &self.flwt())
            .field("lvm", &self.lvm())
            .field("icache", &self.icache())
            .field("dcache", &self.dcache())
            .field("prefete", &self.prefete())
            .field("crst", &self.crst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc FLWT"]
    #[inline(always)]
    pub fn flwt(&mut self) -> FlwtW<'_, FrmcSpec> {
        FlwtW::new(self, 0)
    }
    #[doc = "Bit 8 - desc LVM"]
    #[inline(always)]
    pub fn lvm(&mut self) -> LvmW<'_, FrmcSpec> {
        LvmW::new(self, 8)
    }
    #[doc = "Bit 16 - desc ICACHE"]
    #[inline(always)]
    pub fn icache(&mut self) -> IcacheW<'_, FrmcSpec> {
        IcacheW::new(self, 16)
    }
    #[doc = "Bit 17 - desc DCACHE"]
    #[inline(always)]
    pub fn dcache(&mut self) -> DcacheW<'_, FrmcSpec> {
        DcacheW::new(self, 17)
    }
    #[doc = "Bit 18 - desc PREFETE"]
    #[inline(always)]
    pub fn prefete(&mut self) -> PrefeteW<'_, FrmcSpec> {
        PrefeteW::new(self, 18)
    }
    #[doc = "Bit 19 - desc CRST"]
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<'_, FrmcSpec> {
        CrstW::new(self, 19)
    }
}
#[doc = "desc FRMC\n\nYou can [`read`](crate::Reg::read) this register and get [`frmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmcSpec;
impl crate::RegisterSpec for FrmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmc::R`](R) reader structure"]
impl crate::Readable for FrmcSpec {}
#[doc = "`write(|w| ..)` method takes [`frmc::W`](W) writer structure"]
impl crate::Writable for FrmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRMC to value 0"]
impl crate::Resettable for FrmcSpec {}
