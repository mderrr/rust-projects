use users::{get_user_by_uid, get_current_uid};
use std::process::{Command, Stdio};
use ansi_term::Colour::*;
use std::io::prelude::*;
use ansi_term::Style;
use std::env;
use std::fs;
use std::io;
use std::i32;
use std::path::Path;

const OUTDATED_FILE_NAME: &str = "out-dated";


const PROGRAM_NAME: &str        = "Rust Package Manager";
const PROGRAM_VERSION: &str     = "0.3.0";
const PROGRAM_DESCRIPTION: &str = "an Arch User Repository package manager written in Rust";

const STRING_USAGE: &str   = "Usage:";
const STRING_OPTIONS: &str = "Options:";

const SUFFIX_TEMP: &'static str = "-temp";
const AUR_PKGBUILD_URL: &'static str = "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=";
const ERROR_MESSAGE_FAILED_TO_EXECUTE: &'static str = "Command failed to execute";

const CHAR_ARROW: &'static str = "❯";

const ERROR_INFO_FF: [&'static str; 6] = [ "FF"
                                         , "Mismatched exit and error codes"
                                         , ""
                                         , "This error is raised in rpm when the program exits with an error but said error does not have an assigned error code"
                                         , "An error on my part"
                                         , "None, if you got this error code is because I screwed up" ];

const ERROR_INFO_00: [&'static str; 6] = [ "00"
                                         , "Permission denied"
                                         , "Permission denied (os error 13)"
                                         , "This error is raised in the directory creation proccess due to lack of permissions"
                                         , "All directories created in rpm are accesible without special permissions, therefore the likely cause is a mistype of the directory name"
                                         , "None, if you got this error code I really screwed up" ];

const ERROR_INFO_10: [&'static str; 6] = [ "10"
                                         , "Curl Error"
                                         , "Curl error code 6"
                                         , "This error is raised in the database syncing when rpm uses cURL to get the latest version of all packages"
                                         , "The likely reason is a lack of network connection"
                                         , "Check your internet connection" ];

const ERROR_INFO_20: [&'static str; 6] = [ "20"
                                         , "Grep pkgver/pkgrel Reading Error"
                                         , "Grep error code 1"
                                         , "This error is raised in the database syncing when rpm tries to grep the pkgver/pkgrel of a package"
                                         , "The likely reason is an invalid / removed package name that can be found as a branch in the AUR"
                                         , "Check if the package is still in the AUR" ];

const ERROR_INFO_30: [&'static str; 6] = [ "30"
                                         , "Casting Bytes To String Error"
                                         , "Invalid UTF-8 Sequence"
                                         , "This error is raised when rpm tries cast a Vec<u8> sequence of bytes to a string, usually when trying to get the output from the stdout of an executed command"
                                         , "The last command executed returned an invalid UTF-8 sequence of bytes"
                                         , "None, if you got this error code I really screwed up" ];

const ERROR_INFO_40: [&'static str; 6] = [ "40"
                                         , "User Input Error"
                                         , "User Entered invalid text"
                                         , "This error is raised when rpm prompts the user for some input and the text cannot be read"
                                         , "An invalid input text"
                                         , "Check that you wrote the allowed text for the prompt" ];

const ERROR_INFO_41: [&'static str; 6] = [ "41"
                                         , "User Input Error"
                                         , "User Entered An Unrecognized Option"
                                         , "This error is raised when rpm prompts the user and the read input does not correspond to a valid option"
                                         , "User inputted an invalid option"
                                         , "Check that you wrote the allowed option for the prompt" ];

const ERROR_INFO_50: [&'static str; 6] = [ "50"
                                         , "Package Deletion Error"
                                         , "Package Deletion Error"
                                         , "This error is raised when rpm tries to remove an installed package but it cannot complete the deletion of the package's info file located in ~/.config/rpm/package_info"
                                         , "rpm can't delete a package's info file"
                                         , "Check that the package info file exists" ];

const CODE_INFORMATION_GREP_PKG_READ: ( i32, [i32; 2] ) = ( 00, [0, 0] );
const CODE_INFORMATION_GIT_CLONE: ( i32, [i32; 2] )     = ( 01, [0, 128] );
const CODE_INFORMATION_CURL: ( i32, [i32; 2] )          = ( 02, [0, 0] );
const CODE_INFORMATION_MKDIR: ( i32, [i32; 2] )         = ( 03, [0, 0] );
const CODE_INFORMATION_MAKEPKG: ( i32, [i32; 2] )       = ( 04, [0, 0] );
const CODE_INFORMATION_LS: ( i32, [i32; 2] )            = ( 05, [0, 0] );
const CODE_INFORMATION_CAT: ( i32, [i32; 2] )           = ( 06, [0, 0] );
const CODE_INFORMATION_PACMAN_RNS: ( i32, [i32; 2] )    = ( 07, [0, 0] );

const DIRECTORY_PATH_TMP: &'static str     = "/tmp/";
const DIRECTORY_PATH_RPM_TMP: &'static str = "/tmp/rpm/";

const DIRECTORY_NAME_RPM: &'static str     = "rpm/";
const DIRECTORY_NAME_UPDATES: &'static str = "updates/";
const DIRECTORY_NAME_CONFIG: &'static str  = ".config/";
const DIRECTORY_NAME_PACKAGE_INFO: &'static str  = "package_info/";

const CONFIG_DIRECTORY: &str       = ".config/rpm/";
const PACKAGE_INFO_DIRECTORY: &str = "package_info/";
const AUR_LINK_TEMPLATE: &str      = "https://aur.archlinux.org/";
const EXTENSION_GIT: &str          = ".git";

const PATH_SUBDIRECTORY_UPDATES: &str = "/tmp/rpm/updates/";

// Some wrappers around ansi_term's calls to make them shorter
fn make_bold( string: &str ) -> ansi_term::ANSIString { Style::new().bold().paint(string) }
fn make_yellow( string: &str ) -> ansi_term::ANSIString { Yellow.bold().paint(string) }
fn make_green( string: &str ) -> ansi_term::ANSIString { Green.bold().paint(string) }
fn make_blue( string: &str ) -> ansi_term::ANSIString { Blue.bold().paint(string) }
fn make_red( string: &str ) -> ansi_term::ANSIString { Red.bold().paint(string) }

const PROGRAM_OPTIONS: [ (&str, &str, &str); 9 ] = [ ( "-V, --version", "Display program version information", "\t" )
                                                   , ( "-h, --help", "Show this help message", "\t\t" ) 
                                                   , ( "-q, --quiet", "Enable quiet mode (print only the necessary info)", "\t\t" )
                                                   , ( "-E, --explain", "Explain an error code", "\t" )
                                                   , ( "-Q, --query", "Display a query of installed packages", "\t\t" )
                                                   , ( "-S, --sync", "Install an AUR package", "\t\t" )
                                                   , ( "-Sy, --refresh", "Refresh AUR database", "\t" )
                                                   , ( "-R, --remove", "Removed an installed package", "\t" )
                                                   , ( "-M, --manage", "Show the update manager prompt", "\t" ) ];

fn print_help_message() {
    println!( "\n {} {} {}, {}\n {} {} {} {}.. {}\n\n {} {}"
            , make_green(CHAR_ARROW), make_green(PROGRAM_NAME), make_bold(PROGRAM_VERSION), PROGRAM_DESCRIPTION
            , make_yellow(CHAR_ARROW), make_yellow(STRING_USAGE), make_bold("rpm"), make_red("[options]"), make_blue("[AUR Link]")
            , make_red(CHAR_ARROW), make_red(STRING_OPTIONS) );

    for option in PROGRAM_OPTIONS.iter() {
        println!( "    {}{}{}", make_bold(option.0), option.2, option.1 );
    }
}

fn explain( error_info: [&'static str; 6] ) {
    println!( "\n{}\n", make_red( &format!(" ❯ Info On Error Code {}", error_info[0]) ) );
    println!( " {} {}", make_yellow("Name:"), error_info[1] );
    println!( " {} {}", make_yellow("Full output:"), error_info[2] );
    println!( " {} {}", make_yellow("Description:"), error_info[3] );
    println!( " {} {}", make_yellow("Cause:"), error_info[4] );
    println!( " {} {}", make_yellow("Solution:"), error_info[5] );
}

fn exit_with_error_code( error_code: i32 ) {
    println!( " {} An {} was encountered while executing the last proccess\n {} Run {} for a more detailed explanation", make_red(CHAR_ARROW), make_red("Error"), make_yellow(CHAR_ARROW), make_yellow( &format!("'rpm --explain {:02X}'", error_code) ) );

    std::process::exit(1);
}

fn match_error_codes( code: i32 ) {
    match code {
        0x00 => explain(ERROR_INFO_00),
        0x10 => explain(ERROR_INFO_10),
        0x20 => explain(ERROR_INFO_20),
        0x30 => explain(ERROR_INFO_30),
        0x40 => explain(ERROR_INFO_40),
        0x41 => explain(ERROR_INFO_41),
        0x50 => explain(ERROR_INFO_50),
        0xFF => explain(ERROR_INFO_FF),
        _    => println!( " {} Error code {} does {} exist, did you mistype?", make_red("❯"), make_red( &format!("'{}'", code) ), make_bold("not") ),
    }

    std::process::exit(1);
}

fn get_error_code_from_exit_code( exit_code: i32, command_id: i32 ) -> i32 {
    let code: i32 = exit_code * command_id;

    match code {
        1 => 0x20,
        12 => 0x10,
        _ => 0xFF,
    }
}

fn assert_command_success( command: &mut Command, command_code_information: ( i32, [i32; 2] ) ) -> String {
    let command_id = command_code_information.0;
    let acceptable_exit_codes = command_code_information.1;

    let command_output = command.output().expect(ERROR_MESSAGE_FAILED_TO_EXECUTE);
    let command_exit_code = command_output.status.code().unwrap();
    let command_output_string = get_string_from_stdout(command_output.stdout);

    //print!("command exit code: {}, ", command_exit_code);
    //print!("command stderr: {}, ", get_string_from_stdout(command_output.stderr));
    //println!("command output: {}", command_output_string);

    if ! acceptable_exit_codes.contains( &command_exit_code ) {
        exit_with_error_code( get_error_code_from_exit_code(command_exit_code, command_id) );
    }

    command_output_string
}

fn get_string_from_stdout( command_stdout: Vec<u8> ) -> String {
    let casted_output_string = String::from_utf8(command_stdout);

    match &casted_output_string {
        Ok(_ok)  => (),
        Err(_err) => exit_with_error_code( 0x30 ),
    };

    let mut output_string = casted_output_string.unwrap();
    output_string.pop(); // Remove the last \n

    output_string
}

fn get_home_directory() -> String {
    let user = get_user_by_uid( get_current_uid() ).unwrap();
    let user_name = user.name().to_str().unwrap();
    let directory_path: String = format!("/home/{}/", user_name);

    directory_path
}

fn create_necessary_directories() {
    let config_directory: &str = &format!("{}{}/", get_home_directory(), DIRECTORY_NAME_CONFIG);
    let rpm_config_directory_path: &str = &[config_directory, DIRECTORY_NAME_RPM].concat();
    

    let mut command_mkdir_tmp = Command::new("mkdir");
    command_mkdir_tmp.current_dir(DIRECTORY_PATH_TMP);
    command_mkdir_tmp.args( &["-p", DIRECTORY_NAME_RPM] );

    assert_command_success(&mut command_mkdir_tmp, CODE_INFORMATION_MKDIR);

    let mut command_mkdir_updates_subdirectory = Command::new("mkdir");
    command_mkdir_updates_subdirectory.current_dir(DIRECTORY_PATH_RPM_TMP);
    command_mkdir_updates_subdirectory.args( &["-p", DIRECTORY_NAME_UPDATES] );

    assert_command_success(&mut command_mkdir_updates_subdirectory, CODE_INFORMATION_MKDIR);

    let mut command_mkdir_rpm_config_directory = Command::new("mkdir");
    command_mkdir_rpm_config_directory.current_dir(config_directory);
    command_mkdir_rpm_config_directory.args( &["-p", DIRECTORY_NAME_RPM] );

    assert_command_success(&mut command_mkdir_rpm_config_directory, CODE_INFORMATION_MKDIR);

    let mut command_mkdir_package_info_subdirectory = Command::new("mkdir");
    command_mkdir_package_info_subdirectory.current_dir(rpm_config_directory_path);
    command_mkdir_package_info_subdirectory.args( &["-p", DIRECTORY_NAME_PACKAGE_INFO] );

    assert_command_success(&mut command_mkdir_package_info_subdirectory, CODE_INFORMATION_MKDIR);
}

fn get_latest_package_version( package_name: &str ) -> String {
    let package_pkgbuild_url: &str = &[AUR_PKGBUILD_URL, package_name].concat();
    let temp_file_name: &str       = &[package_name, SUFFIX_TEMP].concat(); 

    let mut curl_command = Command::new("curl");
    curl_command.current_dir(PATH_SUBDIRECTORY_UPDATES);
    curl_command.args( &["-s", "-o", temp_file_name, package_pkgbuild_url] );

    assert_command_success( &mut curl_command, CODE_INFORMATION_CURL );

    let mut read_pkgver_command = Command::new("grep");
    read_pkgver_command.current_dir(PATH_SUBDIRECTORY_UPDATES);
    read_pkgver_command.args( &[ "-oP", "(?<=pkgver=).*", temp_file_name ] );

    let package_pkgver: &str = &assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );

    let mut read_pkgrel_command = Command::new("grep");
    read_pkgrel_command.current_dir(PATH_SUBDIRECTORY_UPDATES);
    read_pkgrel_command.args( &[ "-oP", "(?<=pkgrel=).*", temp_file_name ] );

    let package_pkgrel: &str = &assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

    format!( "{}-{}", package_pkgver, package_pkgrel )
}

fn save_package_info( package_name: &str ) -> std::io::Result<()> {
    let package_info_directory_path: &str = &[ &get_home_directory(), DIRECTORY_NAME_CONFIG, DIRECTORY_NAME_RPM, DIRECTORY_NAME_PACKAGE_INFO ].concat();
    let package_tmp_subdirectory: &str = &[ DIRECTORY_PATH_RPM_TMP, package_name ].concat();
    let package_info_file_path: &str = &[ package_info_directory_path, package_name ].concat();

    let mut package_info_file = fs::File::create( package_info_file_path )?;

    let mut read_pkgver_command = Command::new("grep");
    read_pkgver_command.current_dir(package_tmp_subdirectory);
    read_pkgver_command.args( &[ "pkgver=", "PKGBUILD" ] );

    let package_pkgver: &str = &assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );

    let mut read_pkgrel_command = Command::new("grep");
    read_pkgrel_command.current_dir(package_tmp_subdirectory);
    read_pkgrel_command.args( &[ "pkgrel=", "PKGBUILD" ] );

    let package_pkgrel: &str = &assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

    let package_info_format = format!("{}\n{}", package_pkgver, package_pkgrel);
    package_info_file.write_all( package_info_format.as_bytes() ).unwrap();

    Ok(())
}

fn flush_stout() {
    io::stdout().flush().unwrap(); // Flush to print immedialty
}

fn send_io_to( send_to_null: bool ) -> std::process::Stdio {
    if send_to_null {
        Stdio::null()
    } else {
        Stdio::inherit()
    }
}

fn get_user_input( prompt_message: &str ) -> String {
    let mut user_input = String::new();

    print!("{}", prompt_message);
    flush_stout();

    match std::io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(_) => exit_with_error_code(0x40),
    }

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    user_input
}

fn sync_package( package_name: &str, be_quiet: bool ) {
    let package_tmp_subdirectory: &str = &[ DIRECTORY_PATH_RPM_TMP, package_name, "/" ].concat();
    let full_aur_repository_link: &str = &[ AUR_LINK_TEMPLATE, package_name, EXTENSION_GIT ].concat();

    let package_repository_exists: bool = Path::new(package_tmp_subdirectory).is_dir();
    if package_repository_exists {
        println!( " {} {} a cloned repository for {} already exists in rpm's /tmp/ directory", make_yellow(CHAR_ARROW), make_yellow("Warning"), make_bold(package_name) );

        let user_input: &str = &get_user_input( &format!(" {} {} {} ", make_yellow(CHAR_ARROW), "Force re-cloning?", make_bold("(y/N)")) );
        match user_input {
            "y" | "Y" | "yes" | "Yes" => {
                let _ = fs::remove_dir_all(package_tmp_subdirectory);
            },
            "n" | "N" | "no" | "No" | "" => {
                ()
            },
            _ => exit_with_error_code(0x41),
        }
    }

    let _ = fs::create_dir_all( package_tmp_subdirectory ); // to store cloned repos

    println!(" {} {} {}'s repository", make_blue(CHAR_ARROW), make_blue("Cloning"), make_bold(package_name));

    let mut git_clone_command = Command::new("git");
    git_clone_command.args( &["clone", "-q", full_aur_repository_link, package_tmp_subdirectory] );
    git_clone_command.stdout( send_io_to(be_quiet) );
    git_clone_command.stderr( send_io_to(be_quiet) );

    assert_command_success( &mut git_clone_command, CODE_INFORMATION_GIT_CLONE );

    println!(" {} {} cloning {}'s repository", make_green(CHAR_ARROW), make_green("Done"), make_bold(package_name));

    println!(" {} {} {} with makepkg", make_blue(CHAR_ARROW), make_blue("Installing"), make_bold(package_name));

    let mut makepkg_command = Command::new("makepkg");
    makepkg_command.args( &["-sirc", "--noconfirm"] );
    makepkg_command.current_dir(package_tmp_subdirectory);
    makepkg_command.stdout( send_io_to(be_quiet) );
    makepkg_command.stderr( send_io_to(be_quiet) );

    assert_command_success( &mut makepkg_command, CODE_INFORMATION_MAKEPKG );

    println!(" {} {} installing {} with makepkg", make_green(CHAR_ARROW), make_green("Done"), make_bold(package_name));

    println!(" {} {} {}'s installation", make_blue(CHAR_ARROW), make_blue("Finishing up"), make_bold(package_name));
    
    let _ = save_package_info( package_name );

    println!(" {} {} finishing up {}'s installation", make_green(CHAR_ARROW), make_green("Done"), make_bold(package_name));
    println!("\n {} {}", make_green(CHAR_ARROW), make_green("Package Installation Complete"));
}

fn remove_package( package_name: &str, be_quiet: bool ) {
    let package_info_directory: &str = &[ &get_home_directory(), DIRECTORY_NAME_CONFIG, DIRECTORY_NAME_RPM, DIRECTORY_NAME_PACKAGE_INFO ].concat();
    let package_info_file_path: &str      = &[ package_info_directory, package_name ].concat();

    let mut ls_command = Command::new("ls");
    ls_command.arg(package_info_directory);

    let ls_output_string = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );
    let ls_output_vector: Vec<&str> = ls_output_string.split("\n").collect();

    if ! ls_output_vector.contains(&package_name) {
        println!( " {} {} the package {} is not installed", make_red(CHAR_ARROW), make_red("Error"), make_bold(package_name) );
        return
    }

    match fs::remove_file(package_info_file_path) {
        Ok(_) => (),
        Err(_) => exit_with_error_code(0x50),
    }

    let mut pacman_rns_command = Command::new("sudo");
    pacman_rns_command.args( &["pacman", "--noconfirm", "-Rns", package_name] );
    pacman_rns_command.stdout( send_io_to(be_quiet) );

    assert_command_success( &mut pacman_rns_command, CODE_INFORMATION_PACMAN_RNS );

    if ! be_quiet {
        println!( "\n {} {} {}", make_green(CHAR_ARROW), make_green(package_name), make_green("was uninstalled successfully") );
    }
}

fn get_number_of_tabs( package_name: &str ) -> String {
    let package_name_length = package_name.len();

    match package_name_length {
        16..=24 => String::from("\t"),
        8..=15  => String::from("\t\t"),
        _       => String::from("\t\t\t")
    }
}

fn get_package_info() -> Vec<(String, String)> {
    let mut package_info: Vec<(String, String)> = Vec::new();

    let package_info_directory: &str = &[ &get_home_directory(), CONFIG_DIRECTORY, PACKAGE_INFO_DIRECTORY ].concat();

    let mut ls_command = Command::new("ls");
    ls_command.arg(package_info_directory);

    let ls_output_string = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );

    for package_name in ls_output_string.split("\n") {
        let package_info_file = &[package_info_directory, package_name].concat();

        let mut read_pkgver_command = Command::new("grep");
        read_pkgver_command.args( &["-oP", "(?<=pkgver=).*", package_info_file] );

        let mut read_pkgrel_command = Command::new("grep");
        read_pkgrel_command.args( &["-oP", "(?<=pkgrel=).*", package_info_file] );

        let read_pkgver_string = assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
        let read_pkgrel_string = assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

        let package_version = format!("{}-{}", read_pkgver_string, read_pkgrel_string);

        package_info.push( (package_name.to_string(), package_version) );
    }

    package_info
}

fn write_to_outdated_packages_file( outdated_packages_info: Vec<(String, String, String)> ) -> std::io::Result<()> {
    let outdated_packages_file_path: &str = &[ &get_home_directory(), CONFIG_DIRECTORY, OUTDATED_FILE_NAME ].concat();

    let mut outdated_packages_file = fs::File::create(outdated_packages_file_path)?;

    for package_info in outdated_packages_info {
        let line: String = [ &package_info.0, " ", &package_info.1, " ", &package_info.2, "\n" ].concat();

        outdated_packages_file.write_all( line.as_bytes() )?;
    }

    Ok(())
}

fn empty_outdated_packages_file() -> std::io::Result<()> {
    let outdated_packages_file_path: &str = &[ &get_home_directory(), CONFIG_DIRECTORY, OUTDATED_FILE_NAME ].concat();

    let mut outdated_packages_file = fs::File::create( outdated_packages_file_path )?;

    outdated_packages_file.write_all( "".as_bytes() )?;

    Ok(())
}

fn refresh_packages( be_quiet: bool ) {
    let package_info: Vec<(String, String)> = get_package_info();
    let mut outdated_packages: Vec<(String, String, String)> = Vec::new();
    let mut package_name: String;
    let mut package_current_version: String;
    let mut package_latest_version: String;
    let exit_message: String;

    if ! be_quiet {
        println!(" {} {} AUR database\n", make_blue(CHAR_ARROW), make_blue("Refreshing"));
    }
    
    for info_tuple in package_info {
        package_name            = info_tuple.0;
        package_current_version = info_tuple.1;

        if ! be_quiet {
            print!("\r                                     "); // Clean output line
            print!("\r {} {} {}...", make_blue(CHAR_ARROW), make_blue("Checking"), make_bold(&package_name));
            flush_stout();
        }

        package_latest_version = get_latest_package_version(&package_name);

        if package_current_version != package_latest_version {
            let mut is_newer_version: bool = false;

            let current_version_segments: Vec<&str> = package_current_version.split(".").collect();
            let latest_version_segments: Vec<&str>  = package_latest_version.split(".").collect();
            
            // Since some packages get pulled directly from their repos it its necesary to check if they are actually a newer version
            for ( i, _ ) in current_version_segments.iter().enumerate() {
                if current_version_segments[i] > latest_version_segments[i] {
                    is_newer_version = true;
                    break;
                }
            }
            
            if ! is_newer_version {
                if ! be_quiet {
                    print!("\r {} {} available for {}:{}{} -> {}\n", make_red(CHAR_ARROW), make_red("Update"), make_bold(&package_name),  get_number_of_tabs(&package_name), (package_current_version), make_bold(&package_latest_version) );
                }

                let package_info_tuple: (String, String, String) = (package_name, package_current_version, package_latest_version);

                outdated_packages.push(package_info_tuple);
            }
        }
    }

    if outdated_packages.len() > 0 {
        exit_message = format!("run {} to install available updates", make_yellow("'rpm -Su'"));
        let _ = write_to_outdated_packages_file(outdated_packages);

    } else {
        exit_message = format!("everything is up to date");
        let _ = empty_outdated_packages_file();
    }

    if ! be_quiet {
        println!("\n {} {} refreshing, {}", make_green(CHAR_ARROW), make_green("Done"), exit_message);
    }
}

fn show_installed_packages( be_quiet: bool, arguments: &mut Vec<String> ) {
    let package_info_directory: &str = &[ &get_home_directory(), CONFIG_DIRECTORY, PACKAGE_INFO_DIRECTORY ].concat();
    let mut tabs: String;

    let mut ls_command = Command::new("ls");
    ls_command.arg(package_info_directory);

    let ls_output_string = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );
    let ls_output_vector: Vec<&str> = ls_output_string.split("\n").collect(); 

    let mut matching_packages: Vec<&str> = Vec::new();

    if arguments.len() > 2 {
        let search: &str =  &arguments[2];

        for package in ls_output_vector.iter() {
            if package.contains(search) {
                matching_packages.push(package);
            }
        }

    } else {
        matching_packages = ls_output_vector;
    }

    if ! be_quiet {
        println!("\n{}\t{}", make_red("[ Package Name ]"), make_red("[ Version ]"));
    }

    for package_name in matching_packages.iter() {
        let package_info_file = &[ package_info_directory, package_name ].concat();

        let mut read_pkgver_command = Command::new("grep");
        read_pkgver_command.args( &["-oP", "(?<=pkgver=).*", package_info_file] );

        let mut read_pkgrel_command = Command::new("grep");
        read_pkgrel_command.args( &["-oP", "(?<=pkgrel=).*", package_info_file] );

        let read_pkgver_string = assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
        let read_pkgrel_string = assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

        if be_quiet {
            tabs = " ".to_string();
        } else {
            tabs = get_number_of_tabs(package_name);
        } 

        println!("{}{}{}-{}", make_bold(package_name), tabs, read_pkgver_string, read_pkgrel_string);
    }
}

fn show_outdated_packages( be_quiet: bool, arguments: &mut Vec<String> ) {
    let outdated_file_path = &[ &get_home_directory(), CONFIG_DIRECTORY, OUTDATED_FILE_NAME ].concat();
    let mut version_spacer: String;
    let mut tabs: String;

    let mut cat_command = Command::new("cat");
    cat_command.arg(outdated_file_path);

    let cat_output_string: String = assert_command_success( &mut cat_command, CODE_INFORMATION_CAT );
    let cat_output_vector: Vec<&str> = cat_output_string.split("\n").collect();

    let mut matching_packages: Vec<&str> = Vec::new();

    if arguments.len() > 2 {
        let search: &str =  &arguments[2];

        for package in cat_output_vector.iter() {
            if package.contains(search) {
                matching_packages.push(package);
            }
        }

    } else {
        matching_packages = cat_output_vector;
    }

    if matching_packages[0] == "" {
        if ! be_quiet {
            println!(" {} {} ", make_green(CHAR_ARROW), make_green("All packages are up to date!"));
        }
        
        return
    }

    if ! be_quiet {
        println!("\n{}\t{}\t\t{}", make_red("[ Package Name ]"), make_red("[ Old ]"), make_red("[ New ]"));
    }
    
    for package_info in matching_packages {
        let package_info_vector: Vec<&str> = package_info.split(" ").collect();
        let package_name: &str             = package_info_vector[0];
        let package_current_version: &str  = package_info_vector[1];
        let package_latest_version: &str   = package_info_vector[2];

        if be_quiet {
            tabs = " ".to_string();
            version_spacer = " -> ".to_string();
        } else {
            tabs = get_number_of_tabs(package_name);
            version_spacer = "\t\t".to_string();
        } 

        println!("{}{}{}{}{}", make_bold(package_name), tabs, package_current_version, version_spacer, package_latest_version);
    }
}

fn update_outdated_packages( be_quiet: bool ) {
    let outdated_file_path = &[ &get_home_directory(), CONFIG_DIRECTORY, OUTDATED_FILE_NAME ].concat();

    let mut cat_command = Command::new("cat");
    cat_command.arg(outdated_file_path);

    let cat_output_string: String = assert_command_success( &mut cat_command, CODE_INFORMATION_CAT );
    let outdated_packages: Vec<&str> = cat_output_string.split("\n").collect();

    if outdated_packages[0] == "" {
        if ! be_quiet {
            println!(" {} {}", make_green(CHAR_ARROW), make_green("No available updates, nothing to do"));
        }

        return
    }
    
    for package_info in outdated_packages {
        let package_info_vector: Vec<&str> = package_info.split(" ").collect();
        let package_name: &str             = package_info_vector[0];
        let package_current_version: &str  = package_info_vector[1];
        let package_latest_version: &str   = package_info_vector[2];

        println!( " {} {} {} from {} to {}", make_blue(CHAR_ARROW), make_blue("Updating"), make_bold(package_name), package_current_version, make_bold(package_latest_version) );
        
        sync_package(package_name, be_quiet);

        println!(" {} {} updating {}", make_green(CHAR_ARROW), make_green("Done"), make_bold(package_name));
    }

    let _ = empty_outdated_packages_file();

    println!("\n {} {}", make_green(CHAR_ARROW), make_green("Done Installing Updates"));
}

fn read_environmental_arguments( arguments: &mut Vec<String>, quiet_mode_enabled: &mut bool ) {
    if arguments.len() > 1 {
        let first_argument: &str = &arguments[1];

        if first_argument.starts_with("-") {
            match first_argument {
                "-h" | "--help"    => print_help_message(),
                "-V" | "--version" => println!( " {} {} version {}", make_green(CHAR_ARROW), make_green(PROGRAM_NAME), make_bold(PROGRAM_VERSION) ),
                "-Q" | "--query"   => show_installed_packages(*quiet_mode_enabled, arguments),
                "-Qu"              => show_outdated_packages(*quiet_mode_enabled, arguments),
                "-R" | "--remove"  => remove_package( &arguments[2], *quiet_mode_enabled ),       
                "-S" | "--sync"    => sync_package( &arguments[2], *quiet_mode_enabled ),
                "-Su"              => update_outdated_packages(*quiet_mode_enabled),
                "-Sy"              => refresh_packages(*quiet_mode_enabled),
                "-Syu"             => { refresh_packages(*quiet_mode_enabled);
                                          update_outdated_packages(*quiet_mode_enabled); },

                "-q" | "--quiet"   => { *quiet_mode_enabled = true;
                                          arguments.remove(0); // Remove the processed argument
                                          read_environmental_arguments( arguments, quiet_mode_enabled); },

                "-E" | "--explain" => { let error_code = i32::from_str_radix(&arguments[2], 16);
                                          match_error_codes( error_code.unwrap() ); },

                _                  => println!(" {} {} option {}, run {} for help ", make_red(CHAR_ARROW), make_red("Invalid"), make_red(first_argument), make_yellow("'rpm --help'")),
            }

        } else {
            sync_package( first_argument, *quiet_mode_enabled );
        }

    } else {
        println!( " {} {}, you need to specify at least a {} for installation", make_red(CHAR_ARROW), make_red("No arguments provided"), make_yellow("package name") );
    }
}

fn main() {
    let mut quiet_mode_enabled: bool = false;
    let mut args: Vec<String> = env::args().collect();

    create_necessary_directories();

    read_environmental_arguments( &mut args, &mut quiet_mode_enabled );
}