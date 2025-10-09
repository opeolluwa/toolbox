use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use dialoguer::FuzzySelect;
use serde::Deserialize;
use serde::Serialize;

use crate::constants::SOURCE_DIR;
use crate::errors::file_system::FsError;
use crate::helpers::{console::LogMessage, file_system::file_exists_in_path};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub force: bool,
    pub base_path: PathBuf,
    pub back_up: bool,
}
impl GeneratorConfig {
    pub fn new(force: bool, base_path: PathBuf, back_up: bool) -> Self {
        Self {
            force,
            base_path,
            back_up,
        }
    }
    pub fn parse_options(command_arguments: &ArgMatches) -> Self {
        let mut base_path = ".".into();
        let mut force: bool = false;
        let mut back_up: bool = false;

        if let Some(force_flag) = command_arguments.get_one::<bool>("force") {
            force = *force_flag;
        };

        if let Some(base_path_flag) = command_arguments.get_one::<String>("path") {
            base_path = base_path_flag.to_owned();
        };

        if let Some(back_up_flag) = command_arguments.get_one::<bool>("backup") {
            back_up = *back_up_flag;
        };

        Self {
            force,
            base_path: Path::new(&base_path).to_path_buf(),
            back_up,
        }
    }
    pub fn generate_readme(&self) -> Result<(), FsError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), "README.md");
        let allow_over_write = self.force;
        let backup_existing_file = self.back_up;

        let desired_path = Path::new(&self.base_path).join("README.md");
        let file_path = desired_path.as_path();
        let backup_path = Path::new(&self.base_path).join("README.md").join(".bak");

        if file_exists && !allow_over_write {
            LogMessage::error("the readme already exist, use the --force flag to overwrite it");
            std::process::exit(1);
        } else if file_exists && allow_over_write && !backup_existing_file {
            let allow_overwrite = Confirm::new()
                .with_prompt("The current readme would not be backed up, do you wish to continue?")
                .interact()
                .unwrap();

            if !allow_overwrite {
                std::process::exit(1);
            }

            Self::create_readme_template(file_path)?
        } else if file_exists && allow_over_write && backup_existing_file {
            let _ = std::fs::copy(file_path, backup_path).map_err(|error| {
                LogMessage::error(&error.to_string());
                std::process::exit(1);
            });
            Self::create_readme_template(file_path)?;
        }

        Self::create_readme_template(file_path)?;

        Ok(())
    }
    pub fn generate_ignore_file(&self) -> Result<(), FsError> {
        let file_exists = file_exists_in_path(self.base_path.as_path(), ".gitignore");
        let allow_over_write = self.force;
        let backup_existing_file = self.back_up;

        let desired_path = Path::new(&self.base_path).join(".gitignore");
        let file_path = desired_path.as_path();
        let backup_path = Path::new(&self.base_path).join(".gitignore").join(".bak");

        if file_exists && !allow_over_write {
            "a .gitignorefile already exist on the selectd path, use the --force flag to overwrite it".to_string();
            std::process::exit(1);
        } else if file_exists && allow_over_write {
            fs::remove_file(file_path).map_err(|err| FsError::OperationError(err.to_string()))?
        } else if file_exists && allow_over_write && backup_existing_file {
            fs::copy(file_path, backup_path)
                .map_err(|err| FsError::OperationError(err.to_string()))?;
            fs::remove_file(file_path).map_err(|err| FsError::OperationError(err.to_string()))?;

            Self::create_git_ignore_template(file_path)?
        }
        Self::create_git_ignore_template(file_path)
    }
    pub fn generate_service(base_path: &PathBuf) {
        let folders = [
            "config",
            "controllers",
            "services",
            "entities",
            "dto",
            "src",
        ];

        // create the base path if not exist
        if !Path::new(base_path).exists() {
            let _ = fs::create_dir(base_path);
        }
        LogMessage::success(&format!(
            "creating new service to path {:?}",
            base_path.canonicalize()
        ));
        let _ = folders
            .into_iter()
            .map(|dir| {
                let path = Path::new(base_path).join(dir);
                let _ = fs::create_dir(path);
            })
            .collect::<Vec<_>>();
    }
    fn create_git_ignore_template(full_path: &Path) -> Result<(), FsError> {
        let supported_technologies = vec![
            "AL",
            "Actionscript",
            "Ada",
            "Agda",
            "Android",
            "AppEngine",
            "AppceleratorTitanium",
            "ArchLinuxPackages",
            "Autotools",
            "C++",
            "C",
            "CFWheels",
            "CMake",
            "CUDA",
            "CakePHP",
            "ChefCookbook",
            "Clojure",
            "CodeIgniter",
            "CommonLisp",
            "Composer",
            "Concrete5",
            "Coq",
            "CraftCMS",
            "D",
            "DM",
            "Dart",
            "Delphi",
            "Drupal",
            "EPiServer",
            "Eagle",
            "Elisp",
            "Elixir",
            "Elm",
            "Erlang",
            "ExpressionEngine",
            "ExtJs",
            "Fancy",
            "Finale",
            "FlaxEngine",
            "ForceDotCom",
            "Fortran",
            "FuelPHP",
            "GWT",
            "Gcov",
            "GitBook",
            "Go",
            "Godot",
            "Gradle",
            "Grails",
            "Haskell",
            "IGORPro",
            "Idris",
            "JBoss",
            "JENKINS_HOME",
            "Java",
            "Jekyll",
            "Joomla",
            "Julia",
            "KiCad",
            "Kohana",
            "Kotlin",
            "LICENSE",
            "LabVIEW",
            "Laravel",
            "Leiningen",
            "LemonStand",
            "Lilypond",
            "Lithium",
            "Lua",
            "Magento",
            "Maven",
            "Mercury",
            "MetaProgrammingSystem",
            "Nanoc",
            "Nim",
            "Node",
            "OCaml",
            "Objective-C",
            "Opa",
            "OpenCart",
            "OracleForms",
            "Packer",
            "Perl",
            "Phalcon",
            "PlayFramework",
            "Plone",
            "Prestashop",
            "Processing",
            "PureScript",
            "Python",
            "Qooxdoo",
            "Qt",
            "R",
            "ROS",
            "Racket",
            "Rails",
            "Raku",
            "RhodesRhomobile",
            "Ruby",
            "Rust",
            "SCons",
            "Sass",
            "Scala",
            "Scheme",
            "Scrivener",
            "Sdcc",
            "SeamGen",
            "SketchUp",
            "Smalltalk",
            "Stella",
            "SugarCRM",
            "Swift",
            "Symfony",
            "SymphonyCMS",
            "TeX",
            "Terraform",
            "Textpattern",
            "TurboGears2",
            "TwinCAT3",
            "Typo3",
            "Unity",
            "UnrealEngine",
            "VVVV",
            "VisualStudio",
            "Waf",
            "WordPress",
            "Xojo",
            "Yeoman",
            "Yii",
            "ZendFramework",
            "Zephir",
        ];

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&supported_technologies)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        if let Some(index) = selection {
            println!("User selected item : {}", supported_technologies[index]);
            let selection = supported_technologies[index];
            Self::generate_ignore_file_to_path(selection, full_path);
        } else {
            std::process::exit(1)
        }
        Ok(())
    }

    fn generate_ignore_file_to_path(technology: &str, path: &Path) {
        let src_path = format!("gitignore/{}.gitignore", technology);
        let file_path = SOURCE_DIR.get_file(src_path);

        if let Some(file_content) = file_path {
            let mut file = File::create(path).unwrap();
            file.write_all(file_content.contents()).unwrap();
        }
    }
    fn create_readme_template(full_path: &Path) -> Result<(), FsError> {
        let content = r#" # Project Title
- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

Project description

## Getting Started

### Dependencies

- Describe any prerequisites, libraries, OS version, etc., needed before installing program.
- ex. Windows 10

### Installing

- How/where to download your program
- Any modifications needed to be made to files/folders

### Executing program

- How to run the program
- Step-by-step bullets

```
code blocks for commands
```

## Documentation

Describe any special instructions that are necessary to install a software package on your computer (if applicable).

## Help

Any advise for common problems or issues.

```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie  
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

- 0.2
  - Various bug fixes and optimizations
  - See [commit change]() or See [release history]()
- 0.1
  - Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
        "#;

        let mut file = std::fs::File::create(full_path)
            .map_err(|err| FsError::OperationError(err.to_string()))?;

        file.write_all(content.as_bytes())
            .map_err(|err| FsError::OperationError(err.to_string()))?;

        Ok(())
    }
}
