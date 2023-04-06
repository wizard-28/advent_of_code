use std::collections::HashMap;
use std::fs;

use chumsky::prelude::*;

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FileType<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum FileType<'a> {
    File(usize),
    Dir(&'a str),
}

fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Command<'src>>> {
    let file = text::int(10)
        .map(|string: &str| string.parse::<usize>().unwrap())
        .padded()
        .then_ignore(none_of('\n').repeated().at_least(1))
        .map(FileType::File);

    let dir = just("dir ")
        .ignore_then(
            none_of('\n')
                .repeated()
                .at_least(1)
                .collect::<String>()
                .slice(),
        )
        .map(FileType::Dir);

    let ls = just("ls")
        .ignore_then(text::newline())
        .ignore_then(file.or(dir).padded().repeated().collect())
        .map(Command::Ls);

    let cd = just("cd ")
        .ignore_then(text::ident().or(just("..")).or(just("/")))
        .map(|path| match path {
            ".." => Command::Cd(Cd::Up),
            _ => Command::Cd(Cd::Down(path)),
        });

    just("$ ")
        .ignore_then(cd.or(ls))
        .padded()
        .repeated()
        .collect()
}

fn get_sizes(data: &[Command]) -> HashMap<String, usize> {
    let mut context = Vec::new();
    let mut sizes = HashMap::new();

    for cmd in data {
        match cmd {
            Command::Cd(Cd::Down(dir)) => {
                context.push(*dir);
            },
            Command::Cd(Cd::Up) => {
                context.pop();
            },
            Command::Ls(files) => {
                for file in files {
                    if let FileType::File(size) = file {
                        for i in 1..=context.len() {
                            let full_path = context[..i].join("/");
                            *sizes.entry(full_path).or_default() += size;
                        }
                    }
                }
            },
        }
    }

    sizes
}

fn part1(sizes: &HashMap<String, usize>) -> usize {
    sizes.values().filter(|&&size| size <= 100_000).sum()
}

fn part2(sizes: &HashMap<String, usize>) -> usize {
    let total_size = sizes.get("/").unwrap();
    let unused_space = 70_000_000 - total_size;
    let size_to_be_deleted = 30_000_000 - unused_space;

    let mut eligible_sizes: Vec<usize> = sizes
        .values()
        .filter(|&&size| size >= size_to_be_deleted)
        .copied()
        .collect();

    eligible_sizes.sort_unstable();

    *eligible_sizes.first().unwrap()
}

pub fn main() {
    let input = fs::read_to_string("data/2022/day7").unwrap();
    let data = parser().parse(&input).into_output().unwrap();
    let sizes = get_sizes(&data);
    println!("part 1: {}", part1(&sizes));
    println!("part 2: {}", part2(&sizes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test() {
        let data = parser().parse(TEST_DATA).into_output().unwrap();
        let sizes = get_sizes(&data);

        assert_eq!(part1(&sizes), 95437);
        assert_eq!(part2(&sizes), 24_933_642);
    }
}
