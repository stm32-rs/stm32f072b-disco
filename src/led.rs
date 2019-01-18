#[macro_export]
macro_rules! orange {
    ($($port:ident)?, $($cs:ident)?) => {
        $(
            $port.pc8.into_push_pull_output($cs)
        )?
    }
}

#[macro_export]
macro_rules! green {
    ($($port:ident)?, $($cs:ident)?) => {
        $(
            $port.pc9.into_push_pull_output($cs)
        )?
    }
}

#[macro_export]
macro_rules! red {
    ($($port:ident)?, $($cs:ident)?) => {
        $(
            $port.pc6.into_push_pull_output($cs)
        )?
    }
}

#[macro_export]
macro_rules! blue {
    ($($port:ident)?, $($cs:ident)?) => {
        $(
            $port.pc7.into_push_pull_output($cs)
        )?
    }
}
