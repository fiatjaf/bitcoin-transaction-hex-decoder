#[derive(Debug)]
pub struct Content {
    pub txid: String,
    pub version: u64,
    pub locktime: u64,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub value: u64,
    pub scriptpubkey: String,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: String,
    pub vout: u64,
    pub scriptsig: String,
    pub sequence: u64,
}
