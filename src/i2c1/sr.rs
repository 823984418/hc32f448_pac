#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `STARTF` reader - desc STARTF"]
pub type StartfR = crate::BitReader;
#[doc = "Field `SLADDR0F` reader - desc SLADDR0F"]
pub type Sladdr0fR = crate::BitReader;
#[doc = "Field `SLADDR1F` reader - desc SLADDR1F"]
pub type Sladdr1fR = crate::BitReader;
#[doc = "Field `TENDF` reader - desc TENDF"]
pub type TendfR = crate::BitReader;
#[doc = "Field `STOPF` reader - desc STOPF"]
pub type StopfR = crate::BitReader;
#[doc = "Field `RFULLF` reader - desc RFULLF"]
pub type RfullfR = crate::BitReader;
#[doc = "Field `TEMPTYF` reader - desc TEMPTYF"]
pub type TemptyfR = crate::BitReader;
#[doc = "Field `ARLOF` reader - desc ARLOF"]
pub type ArlofR = crate::BitReader;
#[doc = "Field `ACKRF` reader - desc ACKRF"]
pub type AckrfR = crate::BitReader;
#[doc = "Field `NACKF` reader - desc NACKF"]
pub type NackfR = crate::BitReader;
#[doc = "Field `TMOUTF` reader - desc TMOUTF"]
pub type TmoutfR = crate::BitReader;
#[doc = "Field `MSL` reader - desc MSL"]
pub type MslR = crate::BitReader;
#[doc = "Field `MSL` writer - desc MSL"]
pub type MslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `TRA` reader - desc TRA"]
pub type TraR = crate::BitReader;
#[doc = "Field `TRA` writer - desc TRA"]
pub type TraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALLF` reader - desc GENCALLF"]
pub type GencallfR = crate::BitReader;
#[doc = "Field `SMBDEFAULTF` reader - desc SMBDEFAULTF"]
pub type SmbdefaultfR = crate::BitReader;
#[doc = "Field `SMBHOSTF` reader - desc SMBHOSTF"]
pub type SmbhostfR = crate::BitReader;
#[doc = "Field `SMBALRTF` reader - desc SMBALRTF"]
pub type SmbalrtfR = crate::BitReader;
#[doc = "Field `TFEMPTY` reader - desc TFEMPTY"]
pub type TfemptyR = crate::BitReader;
#[doc = "Field `TFFULL` reader - desc TFFULL"]
pub type TffullR = crate::BitReader;
#[doc = "Field `RFEMPTY` reader - desc RFEMPTY"]
pub type RfemptyR = crate::BitReader;
#[doc = "Field `RFFULL` reader - desc RFFULL"]
pub type RffullR = crate::BitReader;
#[doc = "Field `TFST` reader - desc TFST"]
pub type TfstR = crate::FieldReader;
#[doc = "Field `RFREQ` reader - desc RFREQ"]
pub type RfreqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc STARTF"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SLADDR0F"]
    #[inline(always)]
    pub fn sladdr0f(&self) -> Sladdr0fR {
        Sladdr0fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SLADDR1F"]
    #[inline(always)]
    pub fn sladdr1f(&self) -> Sladdr1fR {
        Sladdr1fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TENDF"]
    #[inline(always)]
    pub fn tendf(&self) -> TendfR {
        TendfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RFULLF"]
    #[inline(always)]
    pub fn rfullf(&self) -> RfullfR {
        RfullfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TEMPTYF"]
    #[inline(always)]
    pub fn temptyf(&self) -> TemptyfR {
        TemptyfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ARLOF"]
    #[inline(always)]
    pub fn arlof(&self) -> ArlofR {
        ArlofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ACKRF"]
    #[inline(always)]
    pub fn ackrf(&self) -> AckrfR {
        AckrfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc NACKF"]
    #[inline(always)]
    pub fn nackf(&self) -> NackfR {
        NackfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TMOUTF"]
    #[inline(always)]
    pub fn tmoutf(&self) -> TmoutfR {
        TmoutfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc MSL"]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TRA"]
    #[inline(always)]
    pub fn tra(&self) -> TraR {
        TraR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - desc GENCALLF"]
    #[inline(always)]
    pub fn gencallf(&self) -> GencallfR {
        GencallfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTF"]
    #[inline(always)]
    pub fn smbdefaultf(&self) -> SmbdefaultfR {
        SmbdefaultfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc SMBHOSTF"]
    #[inline(always)]
    pub fn smbhostf(&self) -> SmbhostfR {
        SmbhostfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc SMBALRTF"]
    #[inline(always)]
    pub fn smbalrtf(&self) -> SmbalrtfR {
        SmbalrtfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TFEMPTY"]
    #[inline(always)]
    pub fn tfempty(&self) -> TfemptyR {
        TfemptyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TFFULL"]
    #[inline(always)]
    pub fn tffull(&self) -> TffullR {
        TffullR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc RFEMPTY"]
    #[inline(always)]
    pub fn rfempty(&self) -> RfemptyR {
        RfemptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc RFFULL"]
    #[inline(always)]
    pub fn rffull(&self) -> RffullR {
        RffullR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - desc TFST"]
    #[inline(always)]
    pub fn tfst(&self) -> TfstR {
        TfstR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RFREQ"]
    #[inline(always)]
    pub fn rfreq(&self) -> RfreqR {
        RfreqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("startf", &self.startf())
            .field("sladdr0f", &self.sladdr0f())
            .field("sladdr1f", &self.sladdr1f())
            .field("tendf", &self.tendf())
            .field("stopf", &self.stopf())
            .field("rfullf", &self.rfullf())
            .field("temptyf", &self.temptyf())
            .field("arlof", &self.arlof())
            .field("ackrf", &self.ackrf())
            .field("nackf", &self.nackf())
            .field("tmoutf", &self.tmoutf())
            .field("msl", &self.msl())
            .field("busy", &self.busy())
            .field("tra", &self.tra())
            .field("gencallf", &self.gencallf())
            .field("smbdefaultf", &self.smbdefaultf())
            .field("smbhostf", &self.smbhostf())
            .field("smbalrtf", &self.smbalrtf())
            .field("tfempty", &self.tfempty())
            .field("tffull", &self.tffull())
            .field("rfempty", &self.rfempty())
            .field("rffull", &self.rffull())
            .field("tfst", &self.tfst())
            .field("rfreq", &self.rfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - desc MSL"]
    #[inline(always)]
    pub fn msl(&mut self) -> MslW<'_, SrSpec> {
        MslW::new(self, 16)
    }
    #[doc = "Bit 18 - desc TRA"]
    #[inline(always)]
    pub fn tra(&mut self) -> TraW<'_, SrSpec> {
        TraW::new(self, 18)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0x0500_0000"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x0500_0000;
}
