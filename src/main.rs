mod cli;
mod srcipts;
mod utils;

use std::{fs::File, path::Path};

use cli::{Cli, Commands};
use gumdrop::Options;
use srcipts::{profile_1, profile_2};
use utils::Config;

fn main() {
    let opts = Cli::parse_args_default_or_exit();
    let cfg_path = home::home_dir().unwrap().join(".macro_scriptx");
    if !Path::new(&cfg_path).exists() {
        File::create(&cfg_path).unwrap();
        let cfg = Config::new(0);
        cfg.save();
    }
    let profile: usize = Config::load().current_profile;

    if let Some(cmd) = opts.command {
        match cmd {
            Commands::Profile(_) => {
                println!("{profile}")
            }
            Commands::One(_) => match profile {
                0 => profile_1::one(),
                1 => profile_2::one(),
                _ => unreachable!(),
            },
            Commands::Two(_) => match profile {
                0 => profile_1::two(),
                1 => profile_2::two(),
                _ => unreachable!(),
            },
            Commands::Three(_) => match profile {
                0 => profile_1::three(),
                1 => profile_2::three(),
                _ => unreachable!(),
            },
            Commands::Four(_) => match profile {
                0 => profile_1::four(),
                1 => profile_2::four(),
                _ => unreachable!(),
            },
            Commands::Five(_) => match profile {
                0 => profile_1::five(),
                1 => profile_2::five(),
                _ => unreachable!(),
            },
            Commands::Six(_) => match profile {
                0 => profile_1::six(),
                1 => profile_2::six(),
                _ => unreachable!(),
            },
            Commands::Seven(_) => match profile {
                0 => profile_1::seven(),
                1 => profile_2::seven(),
                _ => unreachable!(),
            },
            Commands::Eight(_) => match profile {
                0 => profile_1::eight(),
                1 => profile_2::eight(),
                _ => unreachable!(),
            },
            Commands::Nine(_) => match profile {
                0 => profile_1::nine(),
                1 => profile_2::nine(),
                _ => unreachable!(),
            },
            Commands::Ten(_) => match profile {
                0 => profile_1::ten(),
                1 => profile_2::ten(),
                _ => unreachable!(),
            },
            Commands::Eleven(_) => match profile {
                0 => profile_1::eleven(),
                1 => profile_2::eleven(),
                _ => unreachable!(),
            },
            Commands::Twelve(_) => match profile {
                0 => profile_1::twelve(),
                1 => profile_2::twelve(),
                _ => unreachable!(),
            },
            Commands::Thirteen(_) => match profile {
                0 => profile_1::thirteen(),
                1 => profile_2::thirteen(),
                _ => unreachable!(),
            },
            Commands::Fourteen(_) => match profile {
                0 => profile_1::fourteen(),
                1 => profile_2::fourteen(),
                _ => unreachable!(),
            },
            Commands::Fifteen(_) => match profile {
                0 => profile_1::fifteen(),
                1 => profile_2::fifteen(),
                _ => unreachable!(),
            },
            Commands::Sixteen(_) => match profile {
                0 => profile_1::sixteen(),
                1 => profile_2::sixteen(),
                _ => unreachable!(),
            },
            Commands::Seventeen(_) => match profile {
                0 => profile_1::seventeen(),
                1 => profile_2::seventeen(),
                _ => unreachable!(),
            },
            Commands::Eighteen(_) => match profile {
                0 => profile_1::eighteen(),
                1 => profile_2::eighteen(),
                _ => unreachable!(),
            },
            Commands::Nineteen(_) => match profile {
                0 => profile_1::nineteen(),
                1 => profile_2::nineteen(),
                _ => unreachable!(),
            },
            Commands::Twenty(_) => match profile {
                0 => profile_1::twenty(),
                1 => profile_2::twenty(),
                _ => unreachable!(),
            },
            Commands::Twentyone(_) => match profile {
                0 => profile_1::twenty_one(),
                1 => profile_2::twenty_one(),
                _ => unreachable!(),
            },
            Commands::Twentytwo(_) => match profile {
                0 => profile_1::twenty_two(),
                1 => profile_2::twenty_two(),
                _ => unreachable!(),
            },
        }
    } else {
        println!("{}", opts.self_command_list().unwrap());
    }
}
