//! Instrument Code validation

/// Checks if an instrument code is valid according to the CWR specification
pub fn is_valid_instrument_code(code: &str) -> bool {
    INSTRUMENT_CODES.contains(&code)
}

/// Valid instrument codes from the CWR specification
const INSTRUMENT_CODES: &[&str] = &[
    "ACC", "ALP", "ACL", "AFL", "AHN", "ARC", "ASX", "ALT", "AMP", "BAN", "BAR", "BAS", "BCL", "BDR", "BFL", "BHN",
    "BON", "BRD", "BSN", "BSX", "BTB", "BUG", "CAB", "CAL", "CEL", "CHI", "CLA", "CLV", "CON", "COR", "CRO", "CYM",
    "DBL", "DIG", "DRM", "DSN", "DUL", "EFX", "EGT", "EHN", "EKB", "EOR", "EPF", "ESX", "ETB", "ETR", "EUP", "FEM",
    "FID", "FIF", "FLG", "FLT", "FRH", "GIT", "GLO", "GON", "GUI", "HAR", "HCA", "HCL", "HDR", "HPS", "HRN", "KAZ",
    "KEY", "MAL", "MAN", "MAR", "MIC", "MIN", "MIX", "MOD", "NAR", "OBO", "OCA", "OCL", "ONM", "ORC", "ORG", "OTH",
    "PAN", "PER", "PFC", "PIA", "PIC", "REC", "SAX", "SHN", "SIT", "SNR", "SOP", "STB", "STG", "SUS", "SYN", "TAM",
    "TBL", "TBN", "TEN", "TIM", "TOY", "TRG", "TRM", "TRP", "TSX", "TUB", "TYM", "UKU", "VIB", "VIO", "VLA", "VOC",
    "WAH", "WHI", "WOO", "XYL",
];
