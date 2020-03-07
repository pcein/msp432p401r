#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input"]
    pub pain: PAIN,
    #[doc = "0x02 - Port A Output"]
    pub paout: PAOUT,
    #[doc = "0x04 - Port A Direction"]
    pub padir: PADIR,
    #[doc = "0x06 - Port A Resistor Enable"]
    pub paren: PAREN,
    #[doc = "0x08 - Port A Drive Strength"]
    pub pads: PADS,
    #[doc = "0x0a - Port A Select 0"]
    pub pasel0: PASEL0,
    #[doc = "0x0c - Port A Select 1"]
    pub pasel1: PASEL1,
    #[doc = "0x0e - Port 1 Interrupt Vector Register"]
    pub p1iv: P1IV,
    _reserved8: [u8; 6usize],
    #[doc = "0x16 - Port A Complement Select"]
    pub paselc: PASELC,
    #[doc = "0x18 - Port A Interrupt Edge Select"]
    pub paies: PAIES,
    #[doc = "0x1a - Port A Interrupt Enable"]
    pub paie: PAIE,
    #[doc = "0x1c - Port A Interrupt Flag"]
    pub paifg: PAIFG,
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    pub p2iv: P2IV,
    #[doc = "0x20 - Port B Input"]
    pub pbin: PBIN,
    #[doc = "0x22 - Port B Output"]
    pub pbout: PBOUT,
    #[doc = "0x24 - Port B Direction"]
    pub pbdir: PBDIR,
    #[doc = "0x26 - Port B Resistor Enable"]
    pub pbren: PBREN,
    #[doc = "0x28 - Port B Drive Strength"]
    pub pbds: PBDS,
    #[doc = "0x2a - Port B Select 0"]
    pub pbsel0: PBSEL0,
    #[doc = "0x2c - Port B Select 1"]
    pub pbsel1: PBSEL1,
    #[doc = "0x2e - Port 3 Interrupt Vector Register"]
    pub p3iv: P3IV,
    _reserved21: [u8; 6usize],
    #[doc = "0x36 - Port B Complement Select"]
    pub pbselc: PBSELC,
    #[doc = "0x38 - Port B Interrupt Edge Select"]
    pub pbies: PBIES,
    #[doc = "0x3a - Port B Interrupt Enable"]
    pub pbie: PBIE,
    #[doc = "0x3c - Port B Interrupt Flag"]
    pub pbifg: PBIFG,
    #[doc = "0x3e - Port 4 Interrupt Vector Register"]
    pub p4iv: P4IV,
    #[doc = "0x40 - Port C Input"]
    pub pcin: PCIN,
    #[doc = "0x42 - Port C Output"]
    pub pcout: PCOUT,
    #[doc = "0x44 - Port C Direction"]
    pub pcdir: PCDIR,
    #[doc = "0x46 - Port C Resistor Enable"]
    pub pcren: PCREN,
    #[doc = "0x48 - Port C Drive Strength"]
    pub pcds: PCDS,
    #[doc = "0x4a - Port C Select 0"]
    pub pcsel0: PCSEL0,
    #[doc = "0x4c - Port C Select 1"]
    pub pcsel1: PCSEL1,
    #[doc = "0x4e - Port 5 Interrupt Vector Register"]
    pub p5iv: P5IV,
    _reserved34: [u8; 6usize],
    #[doc = "0x56 - Port C Complement Select"]
    pub pcselc: PCSELC,
    #[doc = "0x58 - Port C Interrupt Edge Select"]
    pub pcies: PCIES,
    #[doc = "0x5a - Port C Interrupt Enable"]
    pub pcie: PCIE,
    #[doc = "0x5c - Port C Interrupt Flag"]
    pub pcifg: PCIFG,
    #[doc = "0x5e - Port 6 Interrupt Vector Register"]
    pub p6iv: P6IV,
    #[doc = "0x60 - Port D Input"]
    pub pdin: PDIN,
    #[doc = "0x62 - Port D Output"]
    pub pdout: PDOUT,
    #[doc = "0x64 - Port D Direction"]
    pub pddir: PDDIR,
    #[doc = "0x66 - Port D Resistor Enable"]
    pub pdren: PDREN,
    #[doc = "0x68 - Port D Drive Strength"]
    pub pdds: PDDS,
    #[doc = "0x6a - Port D Select 0"]
    pub pdsel0: PDSEL0,
    #[doc = "0x6c - Port D Select 1"]
    pub pdsel1: PDSEL1,
    #[doc = "0x6e - Port 7 Interrupt Vector Register"]
    pub p7iv: P7IV,
    _reserved47: [u8; 6usize],
    #[doc = "0x76 - Port D Complement Select"]
    pub pdselc: PDSELC,
    #[doc = "0x78 - Port D Interrupt Edge Select"]
    pub pdies: PDIES,
    #[doc = "0x7a - Port D Interrupt Enable"]
    pub pdie: PDIE,
    #[doc = "0x7c - Port D Interrupt Flag"]
    pub pdifg: PDIFG,
    #[doc = "0x7e - Port 8 Interrupt Vector Register"]
    pub p8iv: P8IV,
    #[doc = "0x80 - Port E Input"]
    pub pein: PEIN,
    #[doc = "0x82 - Port E Output"]
    pub peout: PEOUT,
    #[doc = "0x84 - Port E Direction"]
    pub pedir: PEDIR,
    #[doc = "0x86 - Port E Resistor Enable"]
    pub peren: PEREN,
    #[doc = "0x88 - Port E Drive Strength"]
    pub peds: PEDS,
    #[doc = "0x8a - Port E Select 0"]
    pub pesel0: PESEL0,
    #[doc = "0x8c - Port E Select 1"]
    pub pesel1: PESEL1,
    #[doc = "0x8e - Port 9 Interrupt Vector Register"]
    pub p9iv: P9IV,
    _reserved60: [u8; 6usize],
    #[doc = "0x96 - Port E Complement Select"]
    pub peselc: PESELC,
    #[doc = "0x98 - Port E Interrupt Edge Select"]
    pub peies: PEIES,
    #[doc = "0x9a - Port E Interrupt Enable"]
    pub peie: PEIE,
    #[doc = "0x9c - Port E Interrupt Flag"]
    pub peifg: PEIFG,
    #[doc = "0x9e - Port 10 Interrupt Vector Register"]
    pub p10iv: P10IV,
    _reserved65: [u8; 128usize],
    #[doc = "0x120 - Port J Input"]
    pub pjin: PJIN,
    #[doc = "0x122 - Port J Output"]
    pub pjout: PJOUT,
    #[doc = "0x124 - Port J Direction"]
    pub pjdir: PJDIR,
    #[doc = "0x126 - Port J Resistor Enable"]
    pub pjren: PJREN,
    #[doc = "0x128 - Port J Drive Strength"]
    pub pjds: PJDS,
    #[doc = "0x12a - Port J Select 0"]
    pub pjsel0: PJSEL0,
    #[doc = "0x12c - Port J Select 1"]
    pub pjsel1: PJSEL1,
    _reserved72: [u8; 8usize],
    #[doc = "0x136 - Port J Complement Select"]
    pub pjselc: PJSELC,
}
#[doc = "Port A Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pain](pain) module"]
pub type PAIN = crate::Reg<u16, _PAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIN;
#[doc = "`read()` method returns [pain::R](pain::R) reader structure"]
impl crate::Readable for PAIN {}
#[doc = "Port A Input"]
pub mod pain;
#[doc = "Port A Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paout](paout) module"]
pub type PAOUT = crate::Reg<u16, _PAOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAOUT;
#[doc = "`read()` method returns [paout::R](paout::R) reader structure"]
impl crate::Readable for PAOUT {}
#[doc = "`write(|w| ..)` method takes [paout::W](paout::W) writer structure"]
impl crate::Writable for PAOUT {}
#[doc = "Port A Output"]
pub mod paout;
#[doc = "Port A Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padir](padir) module"]
pub type PADIR = crate::Reg<u16, _PADIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADIR;
#[doc = "`read()` method returns [padir::R](padir::R) reader structure"]
impl crate::Readable for PADIR {}
#[doc = "`write(|w| ..)` method takes [padir::W](padir::W) writer structure"]
impl crate::Writable for PADIR {}
#[doc = "Port A Direction"]
pub mod padir;
#[doc = "Port A Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paren](paren) module"]
pub type PAREN = crate::Reg<u16, _PAREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAREN;
#[doc = "`read()` method returns [paren::R](paren::R) reader structure"]
impl crate::Readable for PAREN {}
#[doc = "`write(|w| ..)` method takes [paren::W](paren::W) writer structure"]
impl crate::Writable for PAREN {}
#[doc = "Port A Resistor Enable"]
pub mod paren;
#[doc = "Port A Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pads](pads) module"]
pub type PADS = crate::Reg<u16, _PADS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PADS;
#[doc = "`read()` method returns [pads::R](pads::R) reader structure"]
impl crate::Readable for PADS {}
#[doc = "`write(|w| ..)` method takes [pads::W](pads::W) writer structure"]
impl crate::Writable for PADS {}
#[doc = "Port A Drive Strength"]
pub mod pads;
#[doc = "Port A Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasel0](pasel0) module"]
pub type PASEL0 = crate::Reg<u16, _PASEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASEL0;
#[doc = "`read()` method returns [pasel0::R](pasel0::R) reader structure"]
impl crate::Readable for PASEL0 {}
#[doc = "`write(|w| ..)` method takes [pasel0::W](pasel0::W) writer structure"]
impl crate::Writable for PASEL0 {}
#[doc = "Port A Select 0"]
pub mod pasel0;
#[doc = "Port A Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasel1](pasel1) module"]
pub type PASEL1 = crate::Reg<u16, _PASEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASEL1;
#[doc = "`read()` method returns [pasel1::R](pasel1::R) reader structure"]
impl crate::Readable for PASEL1 {}
#[doc = "`write(|w| ..)` method takes [pasel1::W](pasel1::W) writer structure"]
impl crate::Writable for PASEL1 {}
#[doc = "Port A Select 1"]
pub mod pasel1;
#[doc = "Port 1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1iv](p1iv) module"]
pub type P1IV = crate::Reg<u16, _P1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IV;
#[doc = "`read()` method returns [p1iv::R](p1iv::R) reader structure"]
impl crate::Readable for P1IV {}
#[doc = "Port 1 Interrupt Vector Register"]
pub mod p1iv;
#[doc = "Port A Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paselc](paselc) module"]
pub type PASELC = crate::Reg<u16, _PASELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASELC;
#[doc = "`read()` method returns [paselc::R](paselc::R) reader structure"]
impl crate::Readable for PASELC {}
#[doc = "`write(|w| ..)` method takes [paselc::W](paselc::W) writer structure"]
impl crate::Writable for PASELC {}
#[doc = "Port A Complement Select"]
pub mod paselc;
#[doc = "Port A Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paies](paies) module"]
pub type PAIES = crate::Reg<u16, _PAIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIES;
#[doc = "`read()` method returns [paies::R](paies::R) reader structure"]
impl crate::Readable for PAIES {}
#[doc = "`write(|w| ..)` method takes [paies::W](paies::W) writer structure"]
impl crate::Writable for PAIES {}
#[doc = "Port A Interrupt Edge Select"]
pub mod paies;
#[doc = "Port A Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paie](paie) module"]
pub type PAIE = crate::Reg<u16, _PAIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIE;
#[doc = "`read()` method returns [paie::R](paie::R) reader structure"]
impl crate::Readable for PAIE {}
#[doc = "`write(|w| ..)` method takes [paie::W](paie::W) writer structure"]
impl crate::Writable for PAIE {}
#[doc = "Port A Interrupt Enable"]
pub mod paie;
#[doc = "Port A Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paifg](paifg) module"]
pub type PAIFG = crate::Reg<u16, _PAIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIFG;
#[doc = "`read()` method returns [paifg::R](paifg::R) reader structure"]
impl crate::Readable for PAIFG {}
#[doc = "`write(|w| ..)` method takes [paifg::W](paifg::W) writer structure"]
impl crate::Writable for PAIFG {}
#[doc = "Port A Interrupt Flag"]
pub mod paifg;
#[doc = "Port 2 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2iv](p2iv) module"]
pub type P2IV = crate::Reg<u16, _P2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IV;
#[doc = "`read()` method returns [p2iv::R](p2iv::R) reader structure"]
impl crate::Readable for P2IV {}
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
#[doc = "Port B Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbin](pbin) module"]
pub type PBIN = crate::Reg<u16, _PBIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIN;
#[doc = "`read()` method returns [pbin::R](pbin::R) reader structure"]
impl crate::Readable for PBIN {}
#[doc = "Port B Input"]
pub mod pbin;
#[doc = "Port B Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbout](pbout) module"]
pub type PBOUT = crate::Reg<u16, _PBOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBOUT;
#[doc = "`read()` method returns [pbout::R](pbout::R) reader structure"]
impl crate::Readable for PBOUT {}
#[doc = "`write(|w| ..)` method takes [pbout::W](pbout::W) writer structure"]
impl crate::Writable for PBOUT {}
#[doc = "Port B Output"]
pub mod pbout;
#[doc = "Port B Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdir](pbdir) module"]
pub type PBDIR = crate::Reg<u16, _PBDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBDIR;
#[doc = "`read()` method returns [pbdir::R](pbdir::R) reader structure"]
impl crate::Readable for PBDIR {}
#[doc = "`write(|w| ..)` method takes [pbdir::W](pbdir::W) writer structure"]
impl crate::Writable for PBDIR {}
#[doc = "Port B Direction"]
pub mod pbdir;
#[doc = "Port B Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbren](pbren) module"]
pub type PBREN = crate::Reg<u16, _PBREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBREN;
#[doc = "`read()` method returns [pbren::R](pbren::R) reader structure"]
impl crate::Readable for PBREN {}
#[doc = "`write(|w| ..)` method takes [pbren::W](pbren::W) writer structure"]
impl crate::Writable for PBREN {}
#[doc = "Port B Resistor Enable"]
pub mod pbren;
#[doc = "Port B Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbds](pbds) module"]
pub type PBDS = crate::Reg<u16, _PBDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBDS;
#[doc = "`read()` method returns [pbds::R](pbds::R) reader structure"]
impl crate::Readable for PBDS {}
#[doc = "`write(|w| ..)` method takes [pbds::W](pbds::W) writer structure"]
impl crate::Writable for PBDS {}
#[doc = "Port B Drive Strength"]
pub mod pbds;
#[doc = "Port B Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel0](pbsel0) module"]
pub type PBSEL0 = crate::Reg<u16, _PBSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSEL0;
#[doc = "`read()` method returns [pbsel0::R](pbsel0::R) reader structure"]
impl crate::Readable for PBSEL0 {}
#[doc = "`write(|w| ..)` method takes [pbsel0::W](pbsel0::W) writer structure"]
impl crate::Writable for PBSEL0 {}
#[doc = "Port B Select 0"]
pub mod pbsel0;
#[doc = "Port B Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel1](pbsel1) module"]
pub type PBSEL1 = crate::Reg<u16, _PBSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSEL1;
#[doc = "`read()` method returns [pbsel1::R](pbsel1::R) reader structure"]
impl crate::Readable for PBSEL1 {}
#[doc = "`write(|w| ..)` method takes [pbsel1::W](pbsel1::W) writer structure"]
impl crate::Writable for PBSEL1 {}
#[doc = "Port B Select 1"]
pub mod pbsel1;
#[doc = "Port 3 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3iv](p3iv) module"]
pub type P3IV = crate::Reg<u16, _P3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IV;
#[doc = "`read()` method returns [p3iv::R](p3iv::R) reader structure"]
impl crate::Readable for P3IV {}
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
#[doc = "Port B Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbselc](pbselc) module"]
pub type PBSELC = crate::Reg<u16, _PBSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBSELC;
#[doc = "`read()` method returns [pbselc::R](pbselc::R) reader structure"]
impl crate::Readable for PBSELC {}
#[doc = "`write(|w| ..)` method takes [pbselc::W](pbselc::W) writer structure"]
impl crate::Writable for PBSELC {}
#[doc = "Port B Complement Select"]
pub mod pbselc;
#[doc = "Port B Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbies](pbies) module"]
pub type PBIES = crate::Reg<u16, _PBIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIES;
#[doc = "`read()` method returns [pbies::R](pbies::R) reader structure"]
impl crate::Readable for PBIES {}
#[doc = "`write(|w| ..)` method takes [pbies::W](pbies::W) writer structure"]
impl crate::Writable for PBIES {}
#[doc = "Port B Interrupt Edge Select"]
pub mod pbies;
#[doc = "Port B Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbie](pbie) module"]
pub type PBIE = crate::Reg<u16, _PBIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIE;
#[doc = "`read()` method returns [pbie::R](pbie::R) reader structure"]
impl crate::Readable for PBIE {}
#[doc = "`write(|w| ..)` method takes [pbie::W](pbie::W) writer structure"]
impl crate::Writable for PBIE {}
#[doc = "Port B Interrupt Enable"]
pub mod pbie;
#[doc = "Port B Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbifg](pbifg) module"]
pub type PBIFG = crate::Reg<u16, _PBIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBIFG;
#[doc = "`read()` method returns [pbifg::R](pbifg::R) reader structure"]
impl crate::Readable for PBIFG {}
#[doc = "`write(|w| ..)` method takes [pbifg::W](pbifg::W) writer structure"]
impl crate::Writable for PBIFG {}
#[doc = "Port B Interrupt Flag"]
pub mod pbifg;
#[doc = "Port 4 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4iv](p4iv) module"]
pub type P4IV = crate::Reg<u16, _P4IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IV;
#[doc = "`read()` method returns [p4iv::R](p4iv::R) reader structure"]
impl crate::Readable for P4IV {}
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
#[doc = "Port C Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcin](pcin) module"]
pub type PCIN = crate::Reg<u16, _PCIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIN;
#[doc = "`read()` method returns [pcin::R](pcin::R) reader structure"]
impl crate::Readable for PCIN {}
#[doc = "Port C Input"]
pub mod pcin;
#[doc = "Port C Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcout](pcout) module"]
pub type PCOUT = crate::Reg<u16, _PCOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCOUT;
#[doc = "`read()` method returns [pcout::R](pcout::R) reader structure"]
impl crate::Readable for PCOUT {}
#[doc = "`write(|w| ..)` method takes [pcout::W](pcout::W) writer structure"]
impl crate::Writable for PCOUT {}
#[doc = "Port C Output"]
pub mod pcout;
#[doc = "Port C Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdir](pcdir) module"]
pub type PCDIR = crate::Reg<u16, _PCDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDIR;
#[doc = "`read()` method returns [pcdir::R](pcdir::R) reader structure"]
impl crate::Readable for PCDIR {}
#[doc = "`write(|w| ..)` method takes [pcdir::W](pcdir::W) writer structure"]
impl crate::Writable for PCDIR {}
#[doc = "Port C Direction"]
pub mod pcdir;
#[doc = "Port C Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcren](pcren) module"]
pub type PCREN = crate::Reg<u16, _PCREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCREN;
#[doc = "`read()` method returns [pcren::R](pcren::R) reader structure"]
impl crate::Readable for PCREN {}
#[doc = "`write(|w| ..)` method takes [pcren::W](pcren::W) writer structure"]
impl crate::Writable for PCREN {}
#[doc = "Port C Resistor Enable"]
pub mod pcren;
#[doc = "Port C Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcds](pcds) module"]
pub type PCDS = crate::Reg<u16, _PCDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDS;
#[doc = "`read()` method returns [pcds::R](pcds::R) reader structure"]
impl crate::Readable for PCDS {}
#[doc = "`write(|w| ..)` method takes [pcds::W](pcds::W) writer structure"]
impl crate::Writable for PCDS {}
#[doc = "Port C Drive Strength"]
pub mod pcds;
#[doc = "Port C Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel0](pcsel0) module"]
pub type PCSEL0 = crate::Reg<u16, _PCSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSEL0;
#[doc = "`read()` method returns [pcsel0::R](pcsel0::R) reader structure"]
impl crate::Readable for PCSEL0 {}
#[doc = "`write(|w| ..)` method takes [pcsel0::W](pcsel0::W) writer structure"]
impl crate::Writable for PCSEL0 {}
#[doc = "Port C Select 0"]
pub mod pcsel0;
#[doc = "Port C Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel1](pcsel1) module"]
pub type PCSEL1 = crate::Reg<u16, _PCSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSEL1;
#[doc = "`read()` method returns [pcsel1::R](pcsel1::R) reader structure"]
impl crate::Readable for PCSEL1 {}
#[doc = "`write(|w| ..)` method takes [pcsel1::W](pcsel1::W) writer structure"]
impl crate::Writable for PCSEL1 {}
#[doc = "Port C Select 1"]
pub mod pcsel1;
#[doc = "Port 5 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5iv](p5iv) module"]
pub type P5IV = crate::Reg<u16, _P5IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IV;
#[doc = "`read()` method returns [p5iv::R](p5iv::R) reader structure"]
impl crate::Readable for P5IV {}
#[doc = "Port 5 Interrupt Vector Register"]
pub mod p5iv;
#[doc = "Port C Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcselc](pcselc) module"]
pub type PCSELC = crate::Reg<u16, _PCSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSELC;
#[doc = "`read()` method returns [pcselc::R](pcselc::R) reader structure"]
impl crate::Readable for PCSELC {}
#[doc = "`write(|w| ..)` method takes [pcselc::W](pcselc::W) writer structure"]
impl crate::Writable for PCSELC {}
#[doc = "Port C Complement Select"]
pub mod pcselc;
#[doc = "Port C Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcies](pcies) module"]
pub type PCIES = crate::Reg<u16, _PCIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIES;
#[doc = "`read()` method returns [pcies::R](pcies::R) reader structure"]
impl crate::Readable for PCIES {}
#[doc = "`write(|w| ..)` method takes [pcies::W](pcies::W) writer structure"]
impl crate::Writable for PCIES {}
#[doc = "Port C Interrupt Edge Select"]
pub mod pcies;
#[doc = "Port C Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcie](pcie) module"]
pub type PCIE = crate::Reg<u16, _PCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIE;
#[doc = "`read()` method returns [pcie::R](pcie::R) reader structure"]
impl crate::Readable for PCIE {}
#[doc = "`write(|w| ..)` method takes [pcie::W](pcie::W) writer structure"]
impl crate::Writable for PCIE {}
#[doc = "Port C Interrupt Enable"]
pub mod pcie;
#[doc = "Port C Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcifg](pcifg) module"]
pub type PCIFG = crate::Reg<u16, _PCIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCIFG;
#[doc = "`read()` method returns [pcifg::R](pcifg::R) reader structure"]
impl crate::Readable for PCIFG {}
#[doc = "`write(|w| ..)` method takes [pcifg::W](pcifg::W) writer structure"]
impl crate::Writable for PCIFG {}
#[doc = "Port C Interrupt Flag"]
pub mod pcifg;
#[doc = "Port 6 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6iv](p6iv) module"]
pub type P6IV = crate::Reg<u16, _P6IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IV;
#[doc = "`read()` method returns [p6iv::R](p6iv::R) reader structure"]
impl crate::Readable for P6IV {}
#[doc = "Port 6 Interrupt Vector Register"]
pub mod p6iv;
#[doc = "Port D Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdin](pdin) module"]
pub type PDIN = crate::Reg<u16, _PDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIN;
#[doc = "`read()` method returns [pdin::R](pdin::R) reader structure"]
impl crate::Readable for PDIN {}
#[doc = "Port D Input"]
pub mod pdin;
#[doc = "Port D Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdout](pdout) module"]
pub type PDOUT = crate::Reg<u16, _PDOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDOUT;
#[doc = "`read()` method returns [pdout::R](pdout::R) reader structure"]
impl crate::Readable for PDOUT {}
#[doc = "`write(|w| ..)` method takes [pdout::W](pdout::W) writer structure"]
impl crate::Writable for PDOUT {}
#[doc = "Port D Output"]
pub mod pdout;
#[doc = "Port D Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddir](pddir) module"]
pub type PDDIR = crate::Reg<u16, _PDDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDDIR;
#[doc = "`read()` method returns [pddir::R](pddir::R) reader structure"]
impl crate::Readable for PDDIR {}
#[doc = "`write(|w| ..)` method takes [pddir::W](pddir::W) writer structure"]
impl crate::Writable for PDDIR {}
#[doc = "Port D Direction"]
pub mod pddir;
#[doc = "Port D Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdren](pdren) module"]
pub type PDREN = crate::Reg<u16, _PDREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDREN;
#[doc = "`read()` method returns [pdren::R](pdren::R) reader structure"]
impl crate::Readable for PDREN {}
#[doc = "`write(|w| ..)` method takes [pdren::W](pdren::W) writer structure"]
impl crate::Writable for PDREN {}
#[doc = "Port D Resistor Enable"]
pub mod pdren;
#[doc = "Port D Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdds](pdds) module"]
pub type PDDS = crate::Reg<u16, _PDDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDDS;
#[doc = "`read()` method returns [pdds::R](pdds::R) reader structure"]
impl crate::Readable for PDDS {}
#[doc = "`write(|w| ..)` method takes [pdds::W](pdds::W) writer structure"]
impl crate::Writable for PDDS {}
#[doc = "Port D Drive Strength"]
pub mod pdds;
#[doc = "Port D Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel0](pdsel0) module"]
pub type PDSEL0 = crate::Reg<u16, _PDSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSEL0;
#[doc = "`read()` method returns [pdsel0::R](pdsel0::R) reader structure"]
impl crate::Readable for PDSEL0 {}
#[doc = "`write(|w| ..)` method takes [pdsel0::W](pdsel0::W) writer structure"]
impl crate::Writable for PDSEL0 {}
#[doc = "Port D Select 0"]
pub mod pdsel0;
#[doc = "Port D Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel1](pdsel1) module"]
pub type PDSEL1 = crate::Reg<u16, _PDSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSEL1;
#[doc = "`read()` method returns [pdsel1::R](pdsel1::R) reader structure"]
impl crate::Readable for PDSEL1 {}
#[doc = "`write(|w| ..)` method takes [pdsel1::W](pdsel1::W) writer structure"]
impl crate::Writable for PDSEL1 {}
#[doc = "Port D Select 1"]
pub mod pdsel1;
#[doc = "Port 7 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7iv](p7iv) module"]
pub type P7IV = crate::Reg<u16, _P7IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7IV;
#[doc = "`read()` method returns [p7iv::R](p7iv::R) reader structure"]
impl crate::Readable for P7IV {}
#[doc = "Port 7 Interrupt Vector Register"]
pub mod p7iv;
#[doc = "Port D Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdselc](pdselc) module"]
pub type PDSELC = crate::Reg<u16, _PDSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSELC;
#[doc = "`read()` method returns [pdselc::R](pdselc::R) reader structure"]
impl crate::Readable for PDSELC {}
#[doc = "`write(|w| ..)` method takes [pdselc::W](pdselc::W) writer structure"]
impl crate::Writable for PDSELC {}
#[doc = "Port D Complement Select"]
pub mod pdselc;
#[doc = "Port D Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdies](pdies) module"]
pub type PDIES = crate::Reg<u16, _PDIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIES;
#[doc = "`read()` method returns [pdies::R](pdies::R) reader structure"]
impl crate::Readable for PDIES {}
#[doc = "`write(|w| ..)` method takes [pdies::W](pdies::W) writer structure"]
impl crate::Writable for PDIES {}
#[doc = "Port D Interrupt Edge Select"]
pub mod pdies;
#[doc = "Port D Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdie](pdie) module"]
pub type PDIE = crate::Reg<u16, _PDIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIE;
#[doc = "`read()` method returns [pdie::R](pdie::R) reader structure"]
impl crate::Readable for PDIE {}
#[doc = "`write(|w| ..)` method takes [pdie::W](pdie::W) writer structure"]
impl crate::Writable for PDIE {}
#[doc = "Port D Interrupt Enable"]
pub mod pdie;
#[doc = "Port D Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdifg](pdifg) module"]
pub type PDIFG = crate::Reg<u16, _PDIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIFG;
#[doc = "`read()` method returns [pdifg::R](pdifg::R) reader structure"]
impl crate::Readable for PDIFG {}
#[doc = "`write(|w| ..)` method takes [pdifg::W](pdifg::W) writer structure"]
impl crate::Writable for PDIFG {}
#[doc = "Port D Interrupt Flag"]
pub mod pdifg;
#[doc = "Port 8 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8iv](p8iv) module"]
pub type P8IV = crate::Reg<u16, _P8IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IV;
#[doc = "`read()` method returns [p8iv::R](p8iv::R) reader structure"]
impl crate::Readable for P8IV {}
#[doc = "Port 8 Interrupt Vector Register"]
pub mod p8iv;
#[doc = "Port E Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pein](pein) module"]
pub type PEIN = crate::Reg<u16, _PEIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEIN;
#[doc = "`read()` method returns [pein::R](pein::R) reader structure"]
impl crate::Readable for PEIN {}
#[doc = "Port E Input"]
pub mod pein;
#[doc = "Port E Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peout](peout) module"]
pub type PEOUT = crate::Reg<u16, _PEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEOUT;
#[doc = "`read()` method returns [peout::R](peout::R) reader structure"]
impl crate::Readable for PEOUT {}
#[doc = "`write(|w| ..)` method takes [peout::W](peout::W) writer structure"]
impl crate::Writable for PEOUT {}
#[doc = "Port E Output"]
pub mod peout;
#[doc = "Port E Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedir](pedir) module"]
pub type PEDIR = crate::Reg<u16, _PEDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEDIR;
#[doc = "`read()` method returns [pedir::R](pedir::R) reader structure"]
impl crate::Readable for PEDIR {}
#[doc = "`write(|w| ..)` method takes [pedir::W](pedir::W) writer structure"]
impl crate::Writable for PEDIR {}
#[doc = "Port E Direction"]
pub mod pedir;
#[doc = "Port E Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peren](peren) module"]
pub type PEREN = crate::Reg<u16, _PEREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEREN;
#[doc = "`read()` method returns [peren::R](peren::R) reader structure"]
impl crate::Readable for PEREN {}
#[doc = "`write(|w| ..)` method takes [peren::W](peren::W) writer structure"]
impl crate::Writable for PEREN {}
#[doc = "Port E Resistor Enable"]
pub mod peren;
#[doc = "Port E Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peds](peds) module"]
pub type PEDS = crate::Reg<u16, _PEDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEDS;
#[doc = "`read()` method returns [peds::R](peds::R) reader structure"]
impl crate::Readable for PEDS {}
#[doc = "`write(|w| ..)` method takes [peds::W](peds::W) writer structure"]
impl crate::Writable for PEDS {}
#[doc = "Port E Drive Strength"]
pub mod peds;
#[doc = "Port E Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesel0](pesel0) module"]
pub type PESEL0 = crate::Reg<u16, _PESEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PESEL0;
#[doc = "`read()` method returns [pesel0::R](pesel0::R) reader structure"]
impl crate::Readable for PESEL0 {}
#[doc = "`write(|w| ..)` method takes [pesel0::W](pesel0::W) writer structure"]
impl crate::Writable for PESEL0 {}
#[doc = "Port E Select 0"]
pub mod pesel0;
#[doc = "Port E Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesel1](pesel1) module"]
pub type PESEL1 = crate::Reg<u16, _PESEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PESEL1;
#[doc = "`read()` method returns [pesel1::R](pesel1::R) reader structure"]
impl crate::Readable for PESEL1 {}
#[doc = "`write(|w| ..)` method takes [pesel1::W](pesel1::W) writer structure"]
impl crate::Writable for PESEL1 {}
#[doc = "Port E Select 1"]
pub mod pesel1;
#[doc = "Port 9 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9iv](p9iv) module"]
pub type P9IV = crate::Reg<u16, _P9IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IV;
#[doc = "`read()` method returns [p9iv::R](p9iv::R) reader structure"]
impl crate::Readable for P9IV {}
#[doc = "Port 9 Interrupt Vector Register"]
pub mod p9iv;
#[doc = "Port E Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peselc](peselc) module"]
pub type PESELC = crate::Reg<u16, _PESELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PESELC;
#[doc = "`read()` method returns [peselc::R](peselc::R) reader structure"]
impl crate::Readable for PESELC {}
#[doc = "`write(|w| ..)` method takes [peselc::W](peselc::W) writer structure"]
impl crate::Writable for PESELC {}
#[doc = "Port E Complement Select"]
pub mod peselc;
#[doc = "Port E Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peies](peies) module"]
pub type PEIES = crate::Reg<u16, _PEIES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEIES;
#[doc = "`read()` method returns [peies::R](peies::R) reader structure"]
impl crate::Readable for PEIES {}
#[doc = "`write(|w| ..)` method takes [peies::W](peies::W) writer structure"]
impl crate::Writable for PEIES {}
#[doc = "Port E Interrupt Edge Select"]
pub mod peies;
#[doc = "Port E Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peie](peie) module"]
pub type PEIE = crate::Reg<u16, _PEIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEIE;
#[doc = "`read()` method returns [peie::R](peie::R) reader structure"]
impl crate::Readable for PEIE {}
#[doc = "`write(|w| ..)` method takes [peie::W](peie::W) writer structure"]
impl crate::Writable for PEIE {}
#[doc = "Port E Interrupt Enable"]
pub mod peie;
#[doc = "Port E Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peifg](peifg) module"]
pub type PEIFG = crate::Reg<u16, _PEIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEIFG;
#[doc = "`read()` method returns [peifg::R](peifg::R) reader structure"]
impl crate::Readable for PEIFG {}
#[doc = "`write(|w| ..)` method takes [peifg::W](peifg::W) writer structure"]
impl crate::Writable for PEIFG {}
#[doc = "Port E Interrupt Flag"]
pub mod peifg;
#[doc = "Port 10 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p10iv](p10iv) module"]
pub type P10IV = crate::Reg<u16, _P10IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10IV;
#[doc = "`read()` method returns [p10iv::R](p10iv::R) reader structure"]
impl crate::Readable for P10IV {}
#[doc = "Port 10 Interrupt Vector Register"]
pub mod p10iv;
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjin](pjin) module"]
pub type PJIN = crate::Reg<u16, _PJIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJIN;
#[doc = "`read()` method returns [pjin::R](pjin::R) reader structure"]
impl crate::Readable for PJIN {}
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjout](pjout) module"]
pub type PJOUT = crate::Reg<u16, _PJOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJOUT;
#[doc = "`read()` method returns [pjout::R](pjout::R) reader structure"]
impl crate::Readable for PJOUT {}
#[doc = "`write(|w| ..)` method takes [pjout::W](pjout::W) writer structure"]
impl crate::Writable for PJOUT {}
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjdir](pjdir) module"]
pub type PJDIR = crate::Reg<u16, _PJDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJDIR;
#[doc = "`read()` method returns [pjdir::R](pjdir::R) reader structure"]
impl crate::Readable for PJDIR {}
#[doc = "`write(|w| ..)` method takes [pjdir::W](pjdir::W) writer structure"]
impl crate::Writable for PJDIR {}
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjren](pjren) module"]
pub type PJREN = crate::Reg<u16, _PJREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJREN;
#[doc = "`read()` method returns [pjren::R](pjren::R) reader structure"]
impl crate::Readable for PJREN {}
#[doc = "`write(|w| ..)` method takes [pjren::W](pjren::W) writer structure"]
impl crate::Writable for PJREN {}
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "Port J Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjds](pjds) module"]
pub type PJDS = crate::Reg<u16, _PJDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJDS;
#[doc = "`read()` method returns [pjds::R](pjds::R) reader structure"]
impl crate::Readable for PJDS {}
#[doc = "`write(|w| ..)` method takes [pjds::W](pjds::W) writer structure"]
impl crate::Writable for PJDS {}
#[doc = "Port J Drive Strength"]
pub mod pjds;
#[doc = "Port J Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel0](pjsel0) module"]
pub type PJSEL0 = crate::Reg<u16, _PJSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSEL0;
#[doc = "`read()` method returns [pjsel0::R](pjsel0::R) reader structure"]
impl crate::Readable for PJSEL0 {}
#[doc = "`write(|w| ..)` method takes [pjsel0::W](pjsel0::W) writer structure"]
impl crate::Writable for PJSEL0 {}
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "Port J Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel1](pjsel1) module"]
pub type PJSEL1 = crate::Reg<u16, _PJSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSEL1;
#[doc = "`read()` method returns [pjsel1::R](pjsel1::R) reader structure"]
impl crate::Readable for PJSEL1 {}
#[doc = "`write(|w| ..)` method takes [pjsel1::W](pjsel1::W) writer structure"]
impl crate::Writable for PJSEL1 {}
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "Port J Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjselc](pjselc) module"]
pub type PJSELC = crate::Reg<u16, _PJSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSELC;
#[doc = "`read()` method returns [pjselc::R](pjselc::R) reader structure"]
impl crate::Readable for PJSELC {}
#[doc = "`write(|w| ..)` method takes [pjselc::W](pjselc::W) writer structure"]
impl crate::Writable for PJSELC {}
#[doc = "Port J Complement Select"]
pub mod pjselc;
