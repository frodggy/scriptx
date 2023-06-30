use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Cli {
    #[options(command)]
    pub command: Option<Commands>,
}

#[derive(Debug, Options)]
pub enum Commands {
    Profile(Opts),
    One(Opts),
    Two(Opts),
    Three(Opts),
    Four(Opts),
    Five(Opts),
    Six(Opts),
    Seven(Opts),
    Eight(Opts),
    Nine(Opts),
    Ten(Opts),
    Eleven(Opts),
    Twelve(Opts),
    Thirteen(Opts),
    Fourteen(Opts),
    Fifteen(Opts),
    Sixteen(Opts),
    Seventeen(Opts),
    Eighteen(Opts),
    Nineteen(Opts),
    Twenty(Opts),
    Twentyone(Opts),
    Twentytwo(Opts),
}

#[derive(Debug, Options)]
pub struct Opts {}
