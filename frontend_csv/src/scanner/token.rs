#[derive(Clone, PartialEq, Debug)]
pub enum CSVTokenType {
    MsgTy,
    SubTy,
    MsgData,
    SubMsgData,
    TlvType,
    TlvData,
    ShortChannelId,
    Sha256,
    U16,
    U32,
    U64,
    ChannelId,
    Signature,
    Point,
    ChainHash,
    Byte,
    BigSize,
    LiteralString,
    Number,
    Tu32,
    Tu64,
    Tlvs,
    Dotdotdot,
    Data,
    //FIXME: suppor EOF
    EOF,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CSVToken {
    pub ty: CSVTokenType,
    pub val: String,
}
