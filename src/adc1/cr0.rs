#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "desc MS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ms {
    #[doc = "0: `0`"]
    SeqaSingleshot = 0,
    #[doc = "1: `1`"]
    SeqaCont = 1,
    #[doc = "2: `10`"]
    SeqaSeqbSingleshot = 2,
    #[doc = "3: `11`"]
    SeqaContSeqbSingleshot = 3,
    #[doc = "4: `100`"]
    SeqaBuf = 4,
    #[doc = "6: `110`"]
    SeqaBufSeqbSingleshot = 6,
}
impl From<Ms> for u8 {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ms {
    type Ux = u8;
}
impl crate::IsEnum for Ms {}
#[doc = "Field `MS` reader - desc MS"]
pub type MsR = crate::FieldReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ms> {
        match self.bits {
            0 => Some(Ms::SeqaSingleshot),
            1 => Some(Ms::SeqaCont),
            2 => Some(Ms::SeqaSeqbSingleshot),
            3 => Some(Ms::SeqaContSeqbSingleshot),
            4 => Some(Ms::SeqaBuf),
            6 => Some(Ms::SeqaBufSeqbSingleshot),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_seqa_singleshot(&self) -> bool {
        *self == Ms::SeqaSingleshot
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_seqa_cont(&self) -> bool {
        *self == Ms::SeqaCont
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_seqa_seqb_singleshot(&self) -> bool {
        *self == Ms::SeqaSeqbSingleshot
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_seqa_cont_seqb_singleshot(&self) -> bool {
        *self == Ms::SeqaContSeqbSingleshot
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_seqa_buf(&self) -> bool {
        *self == Ms::SeqaBuf
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_seqa_buf_seqb_singleshot(&self) -> bool {
        *self == Ms::SeqaBufSeqbSingleshot
    }
}
#[doc = "Field `MS` writer - desc MS"]
pub type MsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn seqa_singleshot(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaSingleshot)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn seqa_cont(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaCont)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn seqa_seqb_singleshot(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaSeqbSingleshot)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn seqa_cont_seqb_singleshot(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaContSeqbSingleshot)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn seqa_buf(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaBuf)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn seqa_buf_seqb_singleshot(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::SeqaBufSeqbSingleshot)
    }
}
#[doc = "desc ACCSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Accsel {
    #[doc = "0: `0`"]
    Resolution12bit = 0,
    #[doc = "1: `1`"]
    Resolution10bit = 1,
    #[doc = "2: `10`"]
    Resolution8bit = 2,
}
impl From<Accsel> for u8 {
    #[inline(always)]
    fn from(variant: Accsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Accsel {
    type Ux = u8;
}
impl crate::IsEnum for Accsel {}
#[doc = "Field `ACCSEL` reader - desc ACCSEL"]
pub type AccselR = crate::FieldReader<Accsel>;
impl AccselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Accsel> {
        match self.bits {
            0 => Some(Accsel::Resolution12bit),
            1 => Some(Accsel::Resolution10bit),
            2 => Some(Accsel::Resolution8bit),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_resolution_12bit(&self) -> bool {
        *self == Accsel::Resolution12bit
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_resolution_10bit(&self) -> bool {
        *self == Accsel::Resolution10bit
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_resolution_8bit(&self) -> bool {
        *self == Accsel::Resolution8bit
    }
}
#[doc = "Field `ACCSEL` writer - desc ACCSEL"]
pub type AccselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Accsel>;
impl<'a, REG> AccselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn resolution_12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Accsel::Resolution12bit)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn resolution_10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Accsel::Resolution10bit)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn resolution_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Accsel::Resolution8bit)
    }
}
#[doc = "Field `CLREN` reader - desc CLREN"]
pub type ClrenR = crate::BitReader;
#[doc = "Field `CLREN` writer - desc CLREN"]
pub type ClrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "desc DFMT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfmt {
    #[doc = "0: `0`"]
    DataalignRight = 0,
    #[doc = "1: `1`"]
    DataalignLeft = 1,
}
impl From<Dfmt> for bool {
    #[inline(always)]
    fn from(variant: Dfmt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFMT` reader - desc DFMT"]
pub type DfmtR = crate::BitReader<Dfmt>;
impl DfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfmt {
        match self.bits {
            false => Dfmt::DataalignRight,
            true => Dfmt::DataalignLeft,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_dataalign_right(&self) -> bool {
        *self == Dfmt::DataalignRight
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_dataalign_left(&self) -> bool {
        *self == Dfmt::DataalignLeft
    }
}
#[doc = "Field `DFMT` writer - desc DFMT"]
pub type DfmtW<'a, REG> = crate::BitWriter<'a, REG, Dfmt>;
impl<'a, REG> DfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dataalign_right(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmt::DataalignRight)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dataalign_left(self) -> &'a mut crate::W<REG> {
        self.variant(Dfmt::DataalignLeft)
    }
}
#[doc = "desc AVCNT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avcnt {
    #[doc = "0: `0`"]
    AvgCnt2 = 0,
    #[doc = "1: `1`"]
    AvgCnt4 = 1,
    #[doc = "1: `1`"]
    AvgCnt8 = 1,
    #[doc = "1: `1`"]
    AvgCnt16 = 1,
    #[doc = "1: `1`"]
    AvgCnt32 = 1,
    #[doc = "1: `1`"]
    AvgCnt64 = 1,
    #[doc = "1: `1`"]
    AvgCnt128 = 1,
    #[doc = "1: `1`"]
    AvgCnt256 = 1,
}
impl From<Avcnt> for u8 {
    #[inline(always)]
    fn from(variant: Avcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avcnt {
    type Ux = u8;
}
impl crate::IsEnum for Avcnt {}
#[doc = "Field `AVCNT` reader - desc AVCNT"]
pub type AvcntR = crate::FieldReader<Avcnt>;
impl AvcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avcnt {
        match self.bits {
            0 => Avcnt::AvgCnt2,
            1 => Avcnt::AvgCnt4,
            1 => Avcnt::AvgCnt8,
            1 => Avcnt::AvgCnt16,
            1 => Avcnt::AvgCnt32,
            1 => Avcnt::AvgCnt64,
            1 => Avcnt::AvgCnt128,
            1 => Avcnt::AvgCnt256,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_avg_cnt2(&self) -> bool {
        *self == Avcnt::AvgCnt2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt4(&self) -> bool {
        *self == Avcnt::AvgCnt4
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt8(&self) -> bool {
        *self == Avcnt::AvgCnt8
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt16(&self) -> bool {
        *self == Avcnt::AvgCnt16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt32(&self) -> bool {
        *self == Avcnt::AvgCnt32
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt64(&self) -> bool {
        *self == Avcnt::AvgCnt64
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt128(&self) -> bool {
        *self == Avcnt::AvgCnt128
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_avg_cnt256(&self) -> bool {
        *self == Avcnt::AvgCnt256
    }
}
#[doc = "Field `AVCNT` writer - desc AVCNT"]
pub type AvcntW<'a, REG> = crate::FieldWriter<'a, REG, 3, Avcnt, crate::Safe>;
impl<'a, REG> AvcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn avg_cnt2(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt4(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt8(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt16(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt32(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt32)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt64(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt64)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt128(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt128)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn avg_cnt256(self) -> &'a mut crate::W<REG> {
        self.variant(Avcnt::AvgCnt256)
    }
}
impl R {
    #[doc = "Bits 0:2 - desc MS"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - desc ACCSEL"]
    #[inline(always)]
    pub fn accsel(&self) -> AccselR {
        AccselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc CLREN"]
    #[inline(always)]
    pub fn clren(&self) -> ClrenR {
        ClrenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DFMT"]
    #[inline(always)]
    pub fn dfmt(&self) -> DfmtR {
        DfmtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc AVCNT"]
    #[inline(always)]
    pub fn avcnt(&self) -> AvcntR {
        AvcntR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0")
            .field("ms", &self.ms())
            .field("accsel", &self.accsel())
            .field("clren", &self.clren())
            .field("dfmt", &self.dfmt())
            .field("avcnt", &self.avcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MS"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Cr0Spec> {
        MsW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc ACCSEL"]
    #[inline(always)]
    pub fn accsel(&mut self) -> AccselW<'_, Cr0Spec> {
        AccselW::new(self, 4)
    }
    #[doc = "Bit 6 - desc CLREN"]
    #[inline(always)]
    pub fn clren(&mut self) -> ClrenW<'_, Cr0Spec> {
        ClrenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DFMT"]
    #[inline(always)]
    pub fn dfmt(&mut self) -> DfmtW<'_, Cr0Spec> {
        DfmtW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc AVCNT"]
    #[inline(always)]
    pub fn avcnt(&mut self) -> AvcntW<'_, Cr0Spec> {
        AvcntW::new(self, 8)
    }
}
#[doc = "desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
