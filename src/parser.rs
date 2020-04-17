use std::collections::VecDeque;
use std::fs::File;
use std::io::{Error as IoError, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

const CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    UnexpectedEof,
    UnexpectedChar(char),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Instruction {
    /// >
    IncrementPtr,
    /// <
    DecrementPtr,
    /// +
    IncrementValue,
    /// -
    DecrementValue,
    /// .
    Output,
    /// ,
    Input,
    /// []
    Loop(VecDeque<Instruction>),
}

pub struct Parser<S: Read + Seek> {
    instructions: VecDeque<Instruction>,
    stream: S,
}

impl Parser<File> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;

        Ok(Self::new(file))
    }
}

impl<S: Read + Seek> Parser<S> {
    pub fn new(stream: S) -> Self {
        Self {
            instructions: VecDeque::new(),
            stream,
        }
    }

    pub fn parse(mut self) -> Result<VecDeque<Instruction>> {
        while let Some(instruction) = self.next()? {
            self.instructions.push_back(instruction);
        }

        Ok(self.instructions)
    }

    fn next(&mut self) -> Result<Option<Instruction>> {
        self.skip()?;

        if let Some(next) = self.next_char()? {
            Ok(Some(self.parse_char(next)?))
        } else {
            Ok(None)
        }
    }

    fn next_char(&mut self) -> Result<Option<char>> {
        let mut buffer = vec![0];
        match self.stream.read_exact(&mut buffer) {
            Ok(()) => Ok(Some(buffer[0] as char)),
            Err(error) if error.kind() == ErrorKind::Interrupted || error.kind() == ErrorKind::UnexpectedEof => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn parse_char(&mut self, c: char) -> Result<Instruction> {
        match c {
            '>' => Ok(Instruction::IncrementPtr),
            '<' => Ok(Instruction::DecrementPtr),
            '+' => Ok(Instruction::IncrementValue),
            '-' => Ok(Instruction::DecrementValue),
            '.' => Ok(Instruction::Output),
            ',' => Ok(Instruction::Input),
            '[' => {
                let mut vec = VecDeque::new();
                loop {
                    self.skip()?;
                    let next_char = self.next_char()?;
                    match next_char {
                        Some(']') => break Ok(Instruction::Loop(vec)),
                        Some(next_char) => vec.push_back(self.parse_char(next_char)?),
                        None => break Err(Error::UnexpectedEof),
                    }
                }
            }
            c => Err(Error::UnexpectedChar(c)),
        }
    }

    fn skip(&mut self) -> Result<()> {
        loop {
            match self.next_char()? {
                Some(c) if !CHARS.contains(&c)=> continue,
                Some(_) => {
                    self.stream.seek(SeekFrom::Current(-1))?;
                    break Ok(());
                }
                None => break Ok(()),
            }
        }
    }
}
