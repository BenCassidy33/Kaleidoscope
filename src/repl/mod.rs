use console::Term;
use derive_more::Display;
use enum_iterator::{Sequence, all};
use miette::IntoDiagnostic;
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use thiserror::Error;

use crate::{
    Lambda, LambdaKind, UnwrapExpressions, UnzipExpressions,
    interpreter::{self, InterpretingError},
    opts::{CreateDefaultOpts, Opts},
};

const HELLO_MESSAGE: &str =
    "Welcome to the Kaleidoscope repl. Use /help for help or /quit to quit.";

#[derive(Error, Debug)]
pub enum ReplError {
    #[error("IO Error!")]
    IoError(#[from] io::Error),

    #[error("Interpreting Error")]
    InterpretingError(#[from] InterpretingError),

    #[error("Unknown error...")]
    Unknown,
}

pub fn run_repl(show_hello: bool) -> Result<(), ReplError> {
    let mut term = Term::stdout();
    let mut opts = Opts::create_default_options();
    let mut history: Vec<String> = Vec::new();

    let mut expression_hist: Vec<Lambda> = Vec::new();

    if show_hello {
        term.write_line(HELLO_MESSAGE)?;
    }

    loop {
        term.write_all(b"> ")?;
        let line = term.read_line()?;
        let line = line.trim();
        history.push(line.to_string());

        if line == "/quit" {
            return Ok(());
        }

        if line.starts_with("/") {
            run_cmd(line, &mut term, &mut opts)?;
            continue;
        }

        let lambda = match Lambda::parse(line).unwrap_expressions() {
            Ok(l) => l,
            Err(e) => {
                write!(term, "{}", e)?;
                continue;
            }
        };

        expression_hist.append(&mut lambda.collect::<Vec<_>>());
        let res = interpreter::interpret(expression_hist.clone(), &mut term, Some(&mut opts));

        if let Err(e) = res {
            let report: miette::Report = e.into();
            writeln!(term, "{:?}", report)?;
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Display, Clone, Copy, PartialEq, Sequence)]
enum CmdKind {
    SetOpt,
    GetOpt,
    Help,
}

impl TryFrom<&str> for CmdKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        for entry in all::<CmdKind>() {
            if entry.to_string().to_lowercase() == value.to_lowercase().replace("/", "") {
                return Ok(entry);
            }
        }

        Err(())
    }
}

struct Cmd {
    kind: CmdKind,
    args: Vec<String>,
}

impl TryFrom<&str> for Cmd {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args: Vec<String> = value.trim().split(" ").map(str::to_string).collect();
        let kind = CmdKind::try_from(args[0].as_str())?;
        let args = args[1..].to_vec();

        Ok(Self { kind, args })
    }
}

pub fn run_cmd(cmd: &str, out: &mut Term, opts: &mut Opts) -> Result<(), ReplError> {
    let Ok(cmd) = Cmd::try_from(cmd) else {
        writeln!(
            out,
            "Invalid command: '{}', cmd. use /help for help.",
            cmd.trim()
        )?;

        return Ok(());
    };

    match cmd.kind {
        CmdKind::Help => {
            writeln!(out, "\nPossible Commands:\n/quit")?;
            for arg in all::<CmdKind>() {
                writeln!(out, "{}", arg.to_string().to_lowercase())?;
            }

            Ok(())
        }

        CmdKind::SetOpt => {
            if cmd.args[0].trim() == "help" {
                for key in opts.keys() {
                    let entry = opts.get(key).unwrap();
                    let valid_opts = entry.get_valid_options_as::<serde_json::Value>().unwrap();

                    let cur = entry
                        .get_current_as::<serde_json::Value>()
                        .unwrap()
                        .to_string();
                    let default = entry
                        .get_default_as::<serde_json::Value>()
                        .unwrap()
                        .to_string();

                    writeln!(
                        out,
                        "Option: {}, current = {:?}, default = {}, valid options = {:?}",
                        key,
                        cur,
                        default,
                        valid_opts
                            .iter()
                            .map(Value::to_string)
                            .collect::<Vec<String>>()
                    )?;
                }
            }

            if let Some(opt) = opts.get_mut(&cmd.args[0]) {
                if cmd.args.len() != 2 {
                    let _ = writeln!(out, "/setopt requires 2 arguments per option.");
                    return Ok(());
                }

                let valid_opts = opt
                    .get_valid_options_as::<Value>()
                    .unwrap()
                    .into_iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<String>>();

                if !valid_opts.contains(&cmd.args[1]) {
                    let _ = writeln!(
                        out,
                        "option, {}, is not valid for entry: {}",
                        cmd.args[1], cmd.args[0]
                    );
                    return Ok(());
                }

                if opt.set_current(cmd.args[1].clone()).is_err() {
                    let _ = writeln!(
                        out,
                        "Failed setting {} for option {}.",
                        cmd.args[1], cmd.args[0]
                    );
                    return Ok(());
                }

                Ok(())
            } else {
                let _ = writeln!(
                    out,
                    "Invalid option! Use /setopt help for a list of options."
                );
                Ok(())
            }
        }

        CmdKind::GetOpt => {
            if cmd.args[0].trim() == "help" {
                for key in opts.keys() {
                    let entry = opts.get(key).unwrap();
                    let valid_opts = entry.get_valid_options_as::<serde_json::Value>().unwrap();

                    let cur = entry
                        .get_current_as::<serde_json::Value>()
                        .unwrap()
                        .to_string();
                    let default = entry
                        .get_default_as::<serde_json::Value>()
                        .unwrap()
                        .to_string();

                    writeln!(
                        out,
                        "Option: {}, current = {:?}, default = {}, valid options = {:?}",
                        key,
                        cur,
                        default,
                        valid_opts
                            .iter()
                            .map(Value::to_string)
                            .collect::<Vec<String>>()
                    )?;
                }
            }

            if let Some(opt) = opts.get(&cmd.args[0]) {
                if cmd.args.len() != 1 {
                    writeln!(out, "/getopt requires only 1 argument per option.")?;
                }

                let cur = opt.get_current_as::<Value>().unwrap().to_string();
                writeln!(out, "{} = {}", cmd.args[0], cur)?;

                Ok(())
            } else {
                writeln!(
                    out,
                    "Invalid option! Use /setopt help for a list of options."
                )?;
                Ok(())
            }
        }
    }
}
