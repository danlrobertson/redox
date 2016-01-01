const CSI: &'static str = "\x1B[";

pub enum OutputEscapeCode {
    CursorUp(u16), // CUU : CSI n A
    CursorDown(u16), // CUD : CSI n B
    CursorRight(u16), // CUF : CSI n C
    CursorLeft(u16), // CUB : CSI n D
    NextLine(u16), // CNL : CSI n E
    PreviousLine(u16), // CPL : CSI n F
    GotoColumn(u16), // CHA : CSI n G
    Goto(u16, u16), // CUP : CSI n ; m H | CSI n ; m f
    GotoStart, // CUP : CSI H | CSI f
    EraseAfter(u16, u16), // ED0 : CSI 0 J | CSI J
    EraseBefore(u16, u16), // ED1 : CSI 1 J
    EraseAll(u16, u16), // ED2 : CSI 2 J
    EraseLineAfter(u16, u16), // EL0 : CSI 0 J | CSI J
    EraseLineBefore(u16, u16), // EL1 : CSI 1 J
    EraseLine(u16, u16), // EL2 : CSI 2 J
    ScrollUp(u16), // SU : CSI n S
    ScrollDown(u16), // SD : CSI n T
    Rendition(u16), // SGR : CSI n m
    SaveCursor, // SCP : CSI s
    RestoreCursor, // RCP : CSI u
    HideCursor, // CSI ?25l
    ShowCursor, // CSI ?25h
    NewLine, // NL
    CarriageReturn, // CR
    None,
}
