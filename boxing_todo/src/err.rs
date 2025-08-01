use std::{error::Error, fmt::{Display,Formatter, Result as Res}};
#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f:&mut Formatter<'_>)->Res{
        write!(f,"Failed to parse file")
    }

}

impl Error for ParseErr {
    fn source(&self)->Option<&(dyn Error+'static)>{
        match self{
            ParseErr::Malformed(s)=>Some(s.as_ref()),
            ParseErr::Empty=>None

        }
    }
}
#[derive(Debug)]

pub struct ReadErr {
    pub child_err:Box<dyn Error>,
}

impl Display for ReadErr {
      fn fmt(&self, f:&mut Formatter<'_>)->Res{
        write!(f,"Failed to read file")
    }

}

impl Error for ReadErr {
        fn source(&self)->Option<&(dyn Error+ 'static)>{
            Some(self.child_err.as_ref() )
        }

}