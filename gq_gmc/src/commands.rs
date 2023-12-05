

#[derive(Debug)]
pub enum ParameterlessCommand {

    GETVER = 1,

    GETCPM = 2,

    HEARTBEAT1 = 3,

    HEARTBEAT2 = 4,

    GETVOLT = 5,

    GETCFG = 7,

    ECFG = 8,

    GETSERIAL = 11,

    POWEROFF = 12,

    CFGUPDATE = 13,

    FACTORYRESET = 20,

    REBOOT  = 21,

    GETDATETIME = 23,

    GETTEMP = 24,

    GETGYRO = 25,

    POWERON = 26,

    WiFiON = 34,

    WiFiOFF = 35,

    WiFiLevel = 37,

    EchoON = 38,

    EchoOFF = 39,

    ALARM1 = 40,

    ALARM0 = 41,

    SPEAKER1 = 42,

    SPEAKER0 = 43,

    GETCPS = 44,

    GETMAXCPS = 45,

    GETCPMH = 46,

    GETCPML = 47

}

/// https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string
impl std::fmt::Display for ParameterlessCommand {
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ButtonKeyCommand {

    /// Back button
    KEY0 = 0,

    /// Down arrow
    KEY1 = 1,

    /// Up arrow
    KEY2 = 2,

    /// Power button 
    KEY3 = 3
    
}

impl std::fmt::Display for ButtonKeyCommand {
    
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}