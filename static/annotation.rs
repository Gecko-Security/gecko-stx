use clarity::vm::representations::Span;
use clarity::vm::ClarityName;
use regex::Regex;
use std::collections::{HashSet, HashMap};

use clap::{App, Arg, SubCommand};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;


#[derive(Debug)]
pub enum AnnotationKind {
    Allow(WarningKind),
    Filter(Vec<ClarityName>),
    FilterAll,
}

struct Number {
    fn new(n: i32) -> Self {
        Number { value: n }
    }

    fn val(&self) -> i32 {
        self.value
    }

    fn add(&mut self, n2: &Number) {
        self.value += n2.val();
    }
}

impl Analyzer {
    type Err = String;

    fn new() -> Self {
        let detector_map = Analyzer::find_detectors();
        let isatty = atty::is(atty::Stream::Stdout);
        Analyzer { detector_map, isatty }
    }

    fn find_detectors() -> HashMap<String, Box<dyn Visitor>> {
        let mut found = HashMap::new();
        let current_dir = env::current_dir().unwrap();
        let detectors_dir = current_dir.join("detectors");

        for entry in fs::read_dir(detectors_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap() == "rs" && path.file_name().unwrap() != "__init__.rs" {
                let module_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let module_path = path.to_str().unwrap().to_string();

                // Load the module and check for Visitor subclasses
                // This is a placeholder for actual dynamic loading and checking
                // Replace this with the actual implementation
                // if let Some(visitor_class) = load_module_and_get_visitor(module_name, module_path) {
                //     found.insert(visitor_class.name(), visitor_class);
                // }
            }
        }

        found
    }

    fn main(&self) {
        let matches = App::new("Static Analyzer for the Clarity language from Stacks")
            .subcommand(
                SubCommand::with_name("lint")
                    .about("Run detectors in a given contract or contracts directory")
                    .arg(Arg::with_name("path").required(true).takes_value(true))
                    .arg(
                        Arg::with_name("filter")
                            .long("filter")
                            .takes_value(true)
                            .multiple(true)
                            .use_delimiter(true)
                            .help("Comma-separated list of detector names to use"),
                    )
                    .arg(
                        Arg::with_name("exclude")
                            .long("exclude")
                            .takes_value(true)
                            .multiple(true)
                            .use_delimiter(true)
                            .help("Comma-separated list of detector names to exclude"),
                    ),
            )
            .subcommand(SubCommand::with_name("detectors").about("List detectors"))
            .get_matches();

        if let Some(matches) = matches.subcommand_matches("lint") {
            let path = matches.value_of("path").unwrap();
            let filters: HashSet<_> = match matches.values_of("filter") {
                Some(values) => values.collect(),
                None => self.detector_map.keys().map(|k| k.as_str()).collect(),
            };

            let excludes: HashSet<_> = match matches.values_of("exclude") {
                Some(values) => values.collect(),
                None => HashSet::new(),
            };

            let detectors = self.get_detectors(filters, excludes);
            if Path::new(path).is_file() && path.ends_with(".clar") {
                self.lint_file(path, &detectors);
            } else {
                for entry in walkdir::WalkDir::new(path) {
                    let entry = entry.unwrap();
                    if entry.path().is_file() && entry.path().extension().unwrap() == "clar" {
                        self.lint_file(entry.path().to_str().unwrap(), &detectors);
                    }
                }
            }
        } else if matches.subcommand_matches("detectors").is_some() {
            self.list_detectors();
        }
    }

    fn get_detectors(&self, filters: HashSet<&str>, excludes: HashSet<&str>) -> Vec<&Box<dyn Visitor>> {
        let mut filtered_names: HashSet<_> = self
            .detector_map
            .keys()
            .filter(|&k| filters.contains(k.as_str()))
            .collect();

        for &exclude in &excludes {
            filtered_names.remove(exclude);
        }

        filtered_names
            .into_iter()
            .map(|name| self.detector_map.get(name).unwrap())
            .collect()
    }

    fn lint_file(&self, filename: &str, lints: &[&Box<dyn Visitor>]) {
        if self.isatty {
            println!(
                "{}====== Linting {}... ======{}",
                TerminalColors::HEADER,
                filename,
                TerminalColors::ENDC
            );
        } else {
            println!("====== Linting {}... ======", filename);
        }

        let source = fs::read_to_string(filename).unwrap();
        let mut runner = LinterRunner::new(source, Some(filename.to_string()));
        runner.add_lints(lints);

        let findings = runner.run();

        // Process findings
    }

    fn list_detectors(&self) {
        let convert_camel_case = |s: &str| -> String {
            let mut result = String::new();
            for (i, c) in s.chars().enumerate() {
                if i == 0 {
                    result.push(c);
                } else if c.is_uppercase() {
                    result.push(' ');
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
            result
        };

        let detectors: Vec<_> = self
            .detector_map
            .keys()
            .map(|s| convert_camel_case(s))
            .collect();

        let max_length = detectors.iter().map(|s| s.len()).max().unwrap();
        let s = max_length / 2 - 4;

        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        if self.isatty {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
        }

        writeln!(
            stdout,
            " ┌{} Detectors {}┐",
            "─".repeat(s),
            "─".repeat(s)
        )
        .unwrap();

        for detector in detectors {
            writeln!(
                stdout,
                " | {}{}|",
                detector,
                " ".repeat(max_length - detector.len() + 1)
            )
            .unwrap();
        }

        writeln!(stdout, " └{}┘", "─".repeat(max_length + 2)).unwrap();

        if self.isatty {
            stdout.reset().unwrap();
        }
    }

    
}
