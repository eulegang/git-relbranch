use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use clap::Parser;
use colored::Colorize;
use git2::{BranchType, Repository, Time};

/// Find relative position of a branch compared to other branches
#[derive(Parser)]
pub struct Args {
    /// Show remote branches as well
    #[clap(short, long)]
    all: bool,

    /// The base branch to compare with
    #[clap(short, long)]
    reference: Option<String>,
}

pub struct Record {
    forward: usize,
    backward: usize,
    name: String,
    author: String,
    time: Time,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let repo = Repository::discover(".")?;

    let target = if let Some(ref reference) = args.reference {
        let branch = repo.find_branch(reference, BranchType::Local)?;
        branch.into_reference()
    } else {
        repo.head()?
    };

    let tname = if let Some(pos) = target.name_bytes().iter().rposition(|b| *b == b'/') {
        &target.name_bytes()[pos + 1..]
    } else {
        target.name_bytes()
    };

    let t_oid = target.target().unwrap();

    let branch_filter = match args.all {
        true => None,
        false => Some(BranchType::Local),
    };

    let mut records = Vec::new();
    for branch in repo.branches(branch_filter)? {
        let Ok((branch, _)) = branch else { continue };
        let Ok(bname) = branch.name_bytes() else { continue };

        if bname != tname {
            let Some(b_oid) = branch.get().target() else { continue };
            let Ok(Some(name)) = branch.name() else { continue };
            let name = name.to_string();

            let mut walk = repo.revwalk()?;
            walk.push(b_oid)?;
            walk.hide(t_oid)?;

            let forward = walk.count();

            let mut walk = repo.revwalk()?;
            walk.push(t_oid)?;
            walk.hide(b_oid)?;

            let backward = walk.count();

            let commit = branch.into_reference().peel_to_commit()?;

            let author = commit.author().name().unwrap_or("-").to_string();
            let time = commit.time();

            records.push(Record {
                forward,
                backward,
                name,
                author,
                time,
            });
        }
    }

    let max_delta = records.iter().map(|x| x.delta_space()).max().unwrap_or(0);
    let max_name = records.iter().map(|x| x.name.len()).max().unwrap_or(0);
    let max_author = records.iter().map(|x| x.author.len()).max().unwrap_or(0);

    for record in records {
        let forward = format!("+{}", record.forward).green();
        let backward = format!("-{}", record.backward).red();
        let name = record.name.yellow();
        let author = record.author.magenta();
        let time = record.chrono().to_rfc3339().cyan();

        let delta_pad = format!("{0:1$}", ' ', (max_delta + 1) - record.delta_space());
        let name_pad = format!("{0:1$}", ' ', (max_name + 1) - record.name.len());
        let author_pad = format!("{0:1$}", ' ', (max_author + 1) - record.author.len());

        let delta = format!("{forward} {backward}");

        println!("{delta}{delta_pad}{name}{name_pad}{author}{author_pad}{time}");
    }

    Ok(())
}

impl Record {
    fn delta_space(&self) -> usize {
        let forward = ((self.forward as f64).log10() as usize) + 1;
        let backward = ((self.backward as f64).log10() as usize) + 1;

        forward + backward + 2
    }

    fn chrono(&self) -> DateTime<Local> {
        let naive = NaiveDateTime::from_timestamp_opt(self.time.seconds(), 0).unwrap();
        DateTime::<Local>::from_local(
            naive,
            FixedOffset::west_opt(self.time.offset_minutes() * 60).unwrap(),
        )
    }
}