use users::{get_user_by_uid, get_current_uid};
use std::process::{Command, Stdio};
use ansi_term::Colour::*;
use std::io::prelude::*;
use ansi_term::Style;
use std::path::Path;
use std::env;
use std::fs;
use std::io;

const PROGRAM_NAME: &str                   = "Rust Package Manager";
const PROGRAM_VERSION: &str                = "1.0";
const PROGRAM_DESCRIPTION: &str            = "an Arch User Repository package manager written in Rust";
const PROGRAM_COMMAND: &str                = "rpm";
const PROGRAM_OPTIONS: [ (&str, &str); 9 ] = [ ( "-V,  --version", "Display program version information")
                                                   , ( "-h,  --help   ", "Show this help message" ) 
                                                   , ( "-q,  --quiet  ", "Enable quiet mode (print only the necessary info)" )
                                                   , ( "-E,  --explain", "Explain an error code" )
                                                   , ( "-Q,  --query  ", "Display a query of installed packages" )
                                                   , ( "-S,  --sync   ", "Install an AUR package" )
                                                   , ( "-Sy, --refresh", "Refresh AUR database" )
                                                   , ( "-R,  --remove ", "Removed an installed package" )
                                                   , ( "-M,  --manage ", "Show the update manager prompt" ) ];

const TEMPLATE_OPTIONS: &str      = "<options>...";
const TEMPLATE_PACKAGE_LINK: &str = "[AUR Link]";

const TEMPLATE_LINK_AUR: &'static str          = "https://aur.archlinux.org/";
const TEMPLATE_LINK_AUR_PKGBUILD: &'static str = "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=";

const FILE_NAME_OUTDATED: &str = "out-dated";

const ERROR_INFO_FF: [&'static str; 5] = [ "FF"
                                         , "Mismatched exit and error codes"
                                         , "This error is raised in rpm when the program exits with an error but said error does not have an assigned error code"
                                         , "An error on my part"
                                         , "None, if you got this error code is because I screwed up" ];

const ERROR_INFO_00: [&'static str; 5] = [ "00"
                                         , "Error Doesn't Exist"
                                         , "Error Doesn't Exist"
                                         , "Error Doesn't Exist"
                                         , "Error Doesn't Exist" ];

const ERROR_INFO_E0: [&'static str; 5] = [ "E0"
                                         , "Curl Error"
                                         , "This error is raised in the database syncing when rpm uses cURL to get the latest version of all packages"
                                         , "The likely reason is a lack of network connection"
                                         , "Check your internet connection" ];

const ERROR_INFO_D0: [&'static str; 5] = [ "D0"
                                         , "Grep pkgver/pkgrel Reading Error"
                                         , "This error is raised in the database syncing when rpm tries to grep the pkgver/pkgrel of a package"
                                         , "The likely reason is an invalid / removed package name that can be found as a branch in the AUR"
                                         , "Check if the package is still in the AUR" ];

const ERROR_INFO_C0: [&'static str; 5] = [ "C0"
                                         , "Invalid UTF-8 Sequence"
                                         , "This error is raised when rpm tries cast a Vec<u8> sequence of bytes to a string, usually when trying to get the output from the stdout of an executed command"
                                         , "The last command executed returned an invalid UTF-8 sequence of bytes"
                                         , "None, if you got this error code I really screwed up" ];

const ERROR_INFO_B0: [&'static str; 5] = [ "B0"
                                         , "User Input Error"
                                         , "This error is raised when rpm prompts the user for some input and the text cannot be read"
                                         , "An invalid input text"
                                         , "Check that you wrote the allowed text for the prompt" ];

const ERROR_INFO_B1: [&'static str; 5] = [ "B1"
                                         , "User Input Error"
                                         , "This error is raised when rpm prompts the user and the read input does not correspond to a valid option"
                                         , "User inputted an invalid option"
                                         , "Check that you wrote the allowed option for the prompt" ];

const ERROR_INFO_A0: [&'static str; 5] = [ "A0"
                                         , "Package Deletion Error"
                                         , "This error is raised when rpm tries to remove an installed package but it cannot complete the deletion of the package's info file located in ~/.config/rpm/package_info"
                                         , "rpm can't delete a package's info file"
                                         , "Check that the package info file exists" ];

const CODE_INFORMATION_GIT_CLONE: ( i32, [i32; 2] )     = ( 01, [0, 128] );
const CODE_INFORMATION_CURL: ( i32, [i32; 2] )          = ( 02, [0, 0] );
const CODE_INFORMATION_MKDIR: ( i32, [i32; 2] )         = ( 03, [0, 0] );
const CODE_INFORMATION_MAKEPKG: ( i32, [i32; 2] )       = ( 04, [0, 0] );
const CODE_INFORMATION_LS: ( i32, [i32; 2] )            = ( 05, [0, 0] );
const CODE_INFORMATION_CAT: ( i32, [i32; 2] )           = ( 06, [0, 0] );
const CODE_INFORMATION_PACMAN_RNS: ( i32, [i32; 2] )    = ( 07, [0, 0] );
const CODE_INFORMATION_GREP_PKG_READ: ( i32, [i32; 2] ) = ( 08, [0, 0] );

const DIRECTORY_PATH_TMP: &'static str     = "/tmp/";
const DIRECTORY_PATH_RPM_TMP: &'static str = "/tmp/rpm/";
const DIRECTORY_PATH_UPDATES: &'static str = "/tmp/rpm/updates/";

const DIRECTORY_NAME_RPM: &'static str          = "rpm/";
const DIRECTORY_NAME_UPDATES: &'static str      = "updates/";
const DIRECTORY_NAME_CONFIG: &'static str       = ".config/";
const DIRECTORY_NAME_PACKAGE_INFO: &'static str = "package_info/";
const DIRECTORY_NAME_RPM_CONFIG: &str           = ".config/rpm/";

const EXTENSION_GIT: &str = ".git";

const SUFFIX_TEMP: &'static str = "-temp";

const STRING_FAILED_TO_EXECUTE: &'static str = "Command failed to execute";
const STRING_PACKAGE_NAME: &'static str      = "package name";
const STRING_EXPLANATION: &str               = "Explanation:";
const STRING_SOLUTION: &'static str          = "Solution:";
const STRING_OPTIONS: &str                   = "Options:";
const STRING_USAGE: &str                     = "Usage:";
const STRING_CAUSE: &'static str             = "Cause:";
const STRING_VERSION_SPACER: &'static str    = " -> ";

const COMMAND_NAME_MAKEPKG: &'static str = "makepkg";
const COMMAND_NAME_MKDIR: &'static str   = "mkdir";
const COMMAND_NAME_CURL: &'static str    = "curl";
const COMMAND_NAME_GREP: &'static str    = "grep";
const COMMAND_NAME_SUDO: &'static str    = "sudo";
const COMMAND_NAME_GIT: &'static str     = "git";
const COMMAND_NAME_CAT: &'static str     = "cat";
const COMMAND_NAME_LS: &'static str      = "ls";

const COMMAND_ARGS_MKDIR_PACKAGE_INFO: [&'static str; 2] = ["-p", DIRECTORY_NAME_PACKAGE_INFO];
const COMMAND_ARGS_SUDO: [&'static str; 3]               = ["pacman", "--noconfirm", "-Rns"];
const COMMAND_ARGS_MKDIR_UPDATES: [&'static str; 2]      = ["-p", DIRECTORY_NAME_UPDATES];
const COMMAND_ARGS_MKDIR_TMP: [&'static str; 2]          = ["-p", DIRECTORY_NAME_RPM];
const COMMAND_ARGS_MKDIR_RPM_CONFIG: [&'static str; 2]   = ["-p", DIRECTORY_NAME_RPM];
const COMMAND_ARGS_GREP_READ_PKGVER: [&'static str; 2]   = ["-oP", "(?<=pkgver=).*"];
const _COMMAND_ARGS_GREP_READ_PKGREL: [&'static str; 2]  = ["-oP", "(?<=pkgrel=).*"];
const COMMAND_ARGS_MAKEPKG: [&'static str; 2]            = ["-sirc", "--noconfirm"];
const COMMAND_ARGS_GREP_SAVE_PKGVER: [&'static str; 2]   = ["pkgver=", "PKGBUILD"];
const COMMAND_ARGS_GREP_SAVE_PKGREL: [&'static str; 2]   = ["pkgrel=", "PKGBUILD"];
const COMMAND_ARGS_GIT_CLONE: [&'static str; 2]          = ["clone", "-q"];
const COMMAND_ARGS_CURL: [&'static str; 2]               = ["-s", "-o"];

const ACTION_NO_UPDATES: &'static str            = "No available updates, nothing to do";
const ACTION_INSTALLATION_COMPLETE: &'static str = "Package Installation Complete";
const ACTION_UNINSTALL_COMPLETE: &'static str    = "was uninstalled successfully";
const ACTION_EVERYTHING_UPDATED: &'static str    = "All packages are up to date!";
const ACTION_DONE_UPDATING: &'static str         = "Done Installing Updates";
const ACTION_NO_ARGUMENTS: &'static str          = "No arguments provided";
const ACTION_FORCE_RECLONING: &'static str       = "Force re-cloning?";
const ACTION_FINISHING_UP: &'static str          = "Finishing up";
const ACTION_REFRESHING: &'static str            = "Refreshing";
const ACTION_INSTALLING: &'static str            = "Installing";
const ACTION_UPDATING: &'static str              = "Updating";
const ACTION_CHECKING: &'static str              = "Checking";
const ACTION_INSTALL: &'static str               = "Install";
const ACTION_WARNING: &'static str               = "Warning";
const ACTION_CLONING: &'static str               = "Cloning";
const ACTION_INVALID: &'static str               = "Invalid";
const ACTION_UPDATE: &'static str                = "Update";
const ACTION_ERROR: &'static str                 = "Error";
const ACTION_DONE: &'static str                  = "Done";
const ACTION_QUIT: &'static str                  = "Quit";

const OPTIONS_Y_N: &'static str = "(y/N)";

const OUTPUT_CLEANER: &'static str = "                                                               ";

const CHAR_NEWLINE: &'static str = "\n";
const CHAR_DOT: &'static str     = ".";
const CHAR_ARROW: &'static str   = "❯";
const CHAR_DASH: &'static str    = "-";
const CHAR_SPACE: &'static str   = " ";
const CHAR_EMPTY: &'static str   = "";

const HEADER_PACKAGE_NAME: &'static str        = "[ Package Name ]";
const HEADER_PACKAGE_VERSION: &'static str     = "[ Version ]";
const HEADER_PACKAGE_VERSION_OLD: &'static str = "[ Old ]";
const HEADER_PACKAGE_VERSION_NEW: &'static str = "[ New ]";

const TITLE_NO_UPDATES: &'static str = "No Updates available";
const TITLE_ONE_UPDATE: &'static str = "1 Update available";

const TABS_4: &'static str = "\t\t\t\t";
const TABS_3: &'static str = "\t\t\t";
const TABS_2: &'static str = "\t\t";
const TABS_1: &'static str = "\t";

// Some wrappers around ansi_term's calls to make them shorter
fn make_bold( string: &str ) -> ansi_term::ANSIString { Style::new().bold().paint(string) }
fn make_yellow( string: &str ) -> ansi_term::ANSIString { Yellow.bold().paint(string) }
fn make_purple( string: &str ) -> ansi_term::ANSIString { Purple.bold().paint(string) }
fn make_green( string: &str ) -> ansi_term::ANSIString { Green.bold().paint(string) }
fn make_blue( string: &str ) -> ansi_term::ANSIString { Blue.bold().paint(string) }
fn make_red( string: &str ) -> ansi_term::ANSIString { Red.bold().paint(string) }

// Flush to print immedialty
fn flush_stout() { io::stdout().flush().unwrap(); } 

fn print_help_message() {
    println!( "\n {arrow} {program_name} {program_version}, {program_description}"
            , arrow               = make_green(CHAR_ARROW)
            , program_name        = make_green(PROGRAM_NAME)
            , program_version     = make_bold(PROGRAM_VERSION)
            , program_description = PROGRAM_DESCRIPTION );

    println!( " {arrow} {program_usage} {program_command} {template_options} {template_link}"
            , arrow            = make_yellow(CHAR_ARROW)
            , program_usage    = make_yellow(STRING_USAGE)
            , program_command  = make_bold(PROGRAM_COMMAND)
            , template_options = make_red(TEMPLATE_OPTIONS) 
            , template_link    = make_blue(TEMPLATE_PACKAGE_LINK) );

    println!( "\n {arrow} {string_options}"
            , arrow          = make_red(CHAR_ARROW)
            , string_options = make_red(STRING_OPTIONS) );

    for option in PROGRAM_OPTIONS.iter() {
        println!( "   {option_flag}\t{option_description}", option_flag = make_bold(option.0), option_description = option.1 );
    }
}

fn center_text( text: &str ) -> String {
    let termsize::Size {cols, ..} = termsize::get().unwrap();
    let spacer_size: usize        = cols as usize;

    format!( "{:^1$}", text, spacer_size )
}

fn display_error_info( error_info: [&'static str; 5] ) {
    let explanation_title: String = center_text( &format!( "❯ Info On {} ❮", error_info[1] ) );
    let error_code: String        = center_text( &format!( "code 0x{}",      error_info[0] ) );

    println!( "\n{}", make_red(&explanation_title) );
    println!( "{}\n", make_purple(&error_code) );

    println!( " {arrow} {title} {content}", arrow = make_yellow(CHAR_ARROW), title = make_yellow(STRING_EXPLANATION), content = error_info[2] );
    println!( " {arrow} {title} {content}", arrow = make_yellow(CHAR_ARROW), title = make_yellow(STRING_CAUSE),       content = error_info[3] );
    println!( " {arrow} {title} {content}", arrow = make_yellow(CHAR_ARROW), title = make_yellow(STRING_SOLUTION),    content = error_info[4] );
}

fn explain_error_code( error_code: i32 ) {
    let error_info: [&'static str; 5] = match error_code {
        0xE0 => ERROR_INFO_E0,
        0xD0 => ERROR_INFO_D0,
        0xC0 => ERROR_INFO_C0,
        0xB0 => ERROR_INFO_B0,
        0xB1 => ERROR_INFO_B1,
        0xA0 => ERROR_INFO_A0,
        0xFF => ERROR_INFO_FF,
        _    => ERROR_INFO_00,
    };

    if error_info == ERROR_INFO_00 {
        println!( " {arrow} Error code {error_code} does {not} exist, did you mistype?"
                , arrow      = make_red("❯")
                , error_code = make_red( &format!("{:02X}", error_code) )
                , not        = make_bold("not") );
        
                std::process::exit(1);
    }

    display_error_info(error_info);
}

fn exit_with_error_code( error_code: i32 ) {
    println!( " {arrow} An {error} was encountered while executing the last proccess" 
            , arrow = make_red(CHAR_ARROW), error = make_red(ACTION_ERROR) );

    println!( " {arrow} Run {command} for a more detailed explanation" 
            , arrow   = make_yellow(CHAR_ARROW)
            , command = make_yellow( &format!("rpm --explain {:02X}", error_code) ) );

    std::process::exit(1);
}

fn get_error_code_from_exit_code( exit_code: i32, command_id: i32 ) -> i32 {
    let error_code: i32 = exit_code * command_id;

    match error_code {
        12 => 0xE0,
        _  => 0xFF,
    }
}

fn assert_command_success( command: &mut Command, command_code_information: ( i32, [i32; 2] ) ) -> String {
    let command_id: i32                 = command_code_information.0;
    let acceptable_exit_codes: [i32; 2] = command_code_information.1;

    let command_output                = command.output().expect(STRING_FAILED_TO_EXECUTE);
    let command_exit_code: i32        = command_output.status.code().unwrap();
    let command_output_string: String = get_string_from_stdout(command_output.stdout);

    // Useful debug info
    // println!("[ Command Info ]\nExit code: {}\nStderr: {}\nOutput: {}", command_exit_code, get_string_from_stdout(command_output.stderr), command_output_string );

    if ! acceptable_exit_codes.contains( &command_exit_code ) {
        exit_with_error_code( get_error_code_from_exit_code(command_exit_code, command_id) );
    }

    command_output_string
}

fn get_string_from_stdout( command_stdout: Vec<u8> ) -> String {
    let converted_string = String::from_utf8(command_stdout);

    match &converted_string {
        Ok(_ok)   => (),
        Err(_err) => exit_with_error_code(0xC0),
    };

    let mut output_string: String = converted_string.unwrap();
    output_string.pop(); // Remove the last \n

    output_string
}

fn get_home_directory() -> String {
    let user: users::User      = get_user_by_uid( get_current_uid() ).unwrap();
    let user_name: &str        = user.name().to_str().unwrap();
    let directory_path: String = format!( "/home/{}/", user_name );

    directory_path
}

fn create_necessary_directories() {
    let config_directory_path: &str     = &format!( "{}{}", get_home_directory(),  DIRECTORY_NAME_CONFIG );
    let rpm_config_directory_path: &str = &format!( "{}{}", config_directory_path, DIRECTORY_NAME_RPM );
    
    let mut command_mkdir_tmp = Command::new(COMMAND_NAME_MKDIR);
    command_mkdir_tmp.current_dir(DIRECTORY_PATH_TMP);
    command_mkdir_tmp.args(&COMMAND_ARGS_MKDIR_TMP);

    let mut command_mkdir_updates = Command::new(COMMAND_NAME_MKDIR);
    command_mkdir_updates.current_dir(DIRECTORY_PATH_RPM_TMP);
    command_mkdir_updates.args(&COMMAND_ARGS_MKDIR_UPDATES);

    let mut command_mkdir_rpm_config = Command::new(COMMAND_NAME_MKDIR);
    command_mkdir_rpm_config.current_dir(config_directory_path);
    command_mkdir_rpm_config.args(&COMMAND_ARGS_MKDIR_RPM_CONFIG);

    let mut command_mkdir_package_info_subdirectory = Command::new(COMMAND_NAME_MKDIR);
    command_mkdir_package_info_subdirectory.current_dir(rpm_config_directory_path);
    command_mkdir_package_info_subdirectory.args(&COMMAND_ARGS_MKDIR_PACKAGE_INFO);

    assert_command_success(&mut command_mkdir_tmp, CODE_INFORMATION_MKDIR);
    assert_command_success(&mut command_mkdir_updates, CODE_INFORMATION_MKDIR);
    assert_command_success(&mut command_mkdir_rpm_config, CODE_INFORMATION_MKDIR);
    assert_command_success(&mut command_mkdir_package_info_subdirectory, CODE_INFORMATION_MKDIR);
}

fn get_latest_package_version( package_name: &str ) -> String {
    let package_pkgbuild_url: String = format!("{}{}", TEMPLATE_LINK_AUR_PKGBUILD, package_name);
    let temp_file_name: String       = format!("{}{}", package_name,     SUFFIX_TEMP); 

    let mut curl_command = Command::new(COMMAND_NAME_CURL);
    curl_command.current_dir(DIRECTORY_PATH_UPDATES);
    curl_command.args(&COMMAND_ARGS_CURL);
    curl_command.args( &[&temp_file_name, &package_pkgbuild_url] );

    let mut read_pkgver_command = Command::new(COMMAND_NAME_GREP);
    read_pkgver_command.current_dir(DIRECTORY_PATH_UPDATES);
    read_pkgver_command.args(&COMMAND_ARGS_GREP_READ_PKGVER);
    read_pkgver_command.arg(&temp_file_name);

    let mut read_pkgrel_command = Command::new(COMMAND_NAME_GREP);
    read_pkgrel_command.current_dir(DIRECTORY_PATH_UPDATES);
    read_pkgrel_command.args( &[ "-oP", "(?<=pkgrel=).*", &temp_file_name ] ); // Can't figure out why can't use the const

    assert_command_success( &mut curl_command, CODE_INFORMATION_CURL );
    let package_pkgver: String = assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
    let package_pkgrel: String = assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

    format!( "{}-{}", package_pkgver, package_pkgrel )
}

fn save_package_info( package_name: &str ) -> std::io::Result<()> {
    let package_info_directory_path: String = format!( "{}{}{}{}", &get_home_directory(), DIRECTORY_NAME_CONFIG, DIRECTORY_NAME_RPM, DIRECTORY_NAME_PACKAGE_INFO );
    let package_tmp_subdirectory: String    = format!( "{}{}", DIRECTORY_PATH_RPM_TMP, package_name );
    let package_info_file_path: String      = format!( "{}{}", package_info_directory_path, package_name );

    let mut package_info_file = fs::File::create(package_info_file_path)?;

    let mut save_pkgver_command = Command::new(COMMAND_NAME_GREP);
    save_pkgver_command.current_dir(&package_tmp_subdirectory);
    save_pkgver_command.args(&COMMAND_ARGS_GREP_SAVE_PKGVER);

    let mut save_pkgrel_command = Command::new(COMMAND_NAME_GREP);
    save_pkgrel_command.current_dir(&package_tmp_subdirectory);
    save_pkgrel_command.args(&COMMAND_ARGS_GREP_SAVE_PKGREL);

    let package_pkgver: String = assert_command_success( &mut save_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
    let package_pkgrel: String = assert_command_success( &mut save_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

    let package_info_format: String = format!("{}\n{}", package_pkgver, package_pkgrel);
    package_info_file.write_all( package_info_format.as_bytes() ).unwrap();

    Ok(())
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
        Ok(_ok)   => (),
        Err(_err) => exit_with_error_code(0xB0),
    }

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }

    user_input
}

fn sync_package( package_name: &str, be_quiet: bool ) {
    let package_tmp_subdirectory: String = format!( "{}{}/", DIRECTORY_PATH_RPM_TMP, package_name );
    let full_aur_repository_link: String = format!( "{}{}{}", TEMPLATE_LINK_AUR, package_name, EXTENSION_GIT );

    let force_clone_prompt: String = format!(" {arrow} {action} {options} "
                                            , arrow   = make_yellow(CHAR_ARROW)
                                            , action  = ACTION_FORCE_RECLONING
                                            , options = make_bold(OPTIONS_Y_N) );

    if Path::new(&package_tmp_subdirectory).is_dir() {
        println!( " {arrow} {action} a cloned repository for {package_name} already exists in rpm's /tmp/ directory"
                , arrow        = make_yellow(CHAR_ARROW)
                , action       = make_yellow(ACTION_WARNING)
                , package_name = make_bold(package_name) );

        let user_input: &str = &get_user_input(&force_clone_prompt);
        match user_input {
            "y" | "Y" | "yes" | "Yes" => {
                let _ = fs::remove_dir_all(&package_tmp_subdirectory);
            },
            "n" | "N" | "no" | "No" | "" => {
                ()
            },
            _ => exit_with_error_code(0xB1),
        }
    }

    let _ = fs::create_dir_all( &package_tmp_subdirectory ); // to store cloned repos

    let mut git_clone_command = Command::new(COMMAND_NAME_GIT);
    git_clone_command.args(&COMMAND_ARGS_GIT_CLONE);
    git_clone_command.args( &[&full_aur_repository_link, &package_tmp_subdirectory] );
    git_clone_command.stdout( send_io_to(be_quiet) );
    git_clone_command.stderr( send_io_to(be_quiet) );

    let mut makepkg_command = Command::new(COMMAND_NAME_MAKEPKG);
    makepkg_command.args( &COMMAND_ARGS_MAKEPKG );
    makepkg_command.current_dir(package_tmp_subdirectory);
    makepkg_command.stdout( send_io_to(be_quiet) );
    makepkg_command.stderr( send_io_to(be_quiet) );

    println!(" {arrow} {action} {package_name}'s repository"
            , arrow        = make_blue(CHAR_ARROW)
            , action       = make_blue(ACTION_CLONING)
            , package_name = make_bold(package_name) );

    assert_command_success( &mut git_clone_command, CODE_INFORMATION_GIT_CLONE );

    println!(" {arrow} {action} cloning {package_name}'s repository"
            , arrow        = make_green(CHAR_ARROW)
            , action       = make_green(ACTION_DONE)
            , package_name = make_bold(package_name) );

    println!(" {arrow} {action} {package_name} with makepkg"
            , arrow        = make_blue(CHAR_ARROW)
            , action       = make_blue(ACTION_INSTALLING)
            , package_name = make_bold(package_name) );

    assert_command_success( &mut makepkg_command, CODE_INFORMATION_MAKEPKG );

    println!(" {arrow} {action} installing {package_name} with makepkg"
            , arrow        = make_green(CHAR_ARROW)
            , action       = make_green(ACTION_DONE)
            , package_name = make_bold(package_name) );

    println!(" {arrow} {action} {package_name}'s installation"
            , arrow        = make_blue(CHAR_ARROW)
            , action       = make_blue(ACTION_FINISHING_UP)
            , package_name = make_bold(package_name));
    
    let _ = save_package_info( package_name );

    println!(" {arrow} {action} finishing up {package_name}'s installation"
            , arrow        = make_green(CHAR_ARROW)
            , action       = make_green(ACTION_DONE)
            , package_name = make_bold(package_name) );

    println!("\n {arrow} {action}", arrow = make_green(CHAR_ARROW), action = make_green(ACTION_INSTALLATION_COMPLETE));
}

fn remove_package( package_name: &str, be_quiet: bool ) {
    let package_info_directory: String = format!( "{}{}{}{}", get_home_directory(), DIRECTORY_NAME_CONFIG, DIRECTORY_NAME_RPM, DIRECTORY_NAME_PACKAGE_INFO );
    let package_info_file_path: String = format!("{}{}", package_info_directory, package_name );

    let mut ls_command = Command::new(COMMAND_NAME_LS);
    ls_command.arg(package_info_directory);

    let ls_output_string = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );
    let ls_output_vector: Vec<&str> = ls_output_string.split(CHAR_NEWLINE).collect();

    if ! ls_output_vector.contains(&package_name) {
        println!( " {arrow} {action} the package {package_name} is not installed"
                , arrow        = make_red(CHAR_ARROW)
                , action       = make_red(ACTION_ERROR)
                , package_name = make_bold(package_name) );

        return
    }

    match fs::remove_file(package_info_file_path) {
        Ok(_) => (),
        Err(_) => exit_with_error_code(0xA0),
    }

    let mut pacman_rns_command = Command::new(COMMAND_NAME_SUDO);
    pacman_rns_command.args(&COMMAND_ARGS_SUDO);
    pacman_rns_command.arg(package_name);
    pacman_rns_command.stdout( send_io_to(be_quiet) );

    assert_command_success( &mut pacman_rns_command, CODE_INFORMATION_PACMAN_RNS );

    if ! be_quiet {
        println!( "\n {arrow} {package_name} {action}"
                , arrow        = make_green(CHAR_ARROW)
                , package_name = make_green(package_name)
                , action       = make_green(ACTION_UNINSTALL_COMPLETE) );
    }
}

fn get_number_of_tabs( package_name: &str ) -> String {
    let package_name_length = package_name.len();

    match package_name_length {
        24..=32 => String::from(TABS_1),
        16..=23 => String::from(TABS_2),
        8..=15  => String::from(TABS_3),
        _       => String::from(TABS_4)
    }
}

fn get_package_info() -> Vec<(String, String)> {
    let mut package_info: Vec<(String, String)> = Vec::new();

    let package_info_directory: String = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, DIRECTORY_NAME_PACKAGE_INFO );

    let mut ls_command = Command::new(COMMAND_NAME_LS);
    ls_command.arg(&package_info_directory);

    let ls_output_string = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );

   

    for package_name in ls_output_string.split(CHAR_NEWLINE) {
        let package_info_file: String = format!("{}{}", &package_info_directory, &package_name );

        let mut read_pkgver_command = Command::new(COMMAND_NAME_GREP);
        read_pkgver_command.args( &COMMAND_ARGS_GREP_READ_PKGVER );
        read_pkgver_command.arg(&package_info_file);
        
        let mut read_pkgrel_command = Command::new(COMMAND_NAME_GREP);
        read_pkgrel_command.args( &["-oP", "(?<=pkgrel=).*", &package_info_file] ); // Why ._.

        let read_pkgver_string = assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
        let read_pkgrel_string = assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

        let package_version = format!( "{}-{}", read_pkgver_string, read_pkgrel_string );

        package_info.push( (package_name.to_string(), package_version) );
    }

    package_info
}

fn write_to_outdated_packages_file( outdated_packages_info: Vec<(String, String, String)> ) -> std::io::Result<()> {
    let outdated_packages_file_path: String = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, FILE_NAME_OUTDATED );

    let mut outdated_packages_file = fs::File::create(outdated_packages_file_path)?;

    for package_info in outdated_packages_info {
        let package_info_line: String = format!( "{} {} {}\n", &package_info.0, &package_info.1, &package_info.2 );

        outdated_packages_file.write_all( package_info_line.as_bytes() )?;
    }

    Ok(())
}

fn empty_outdated_packages_file() -> std::io::Result<()> {
    let outdated_packages_file_path: String = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, FILE_NAME_OUTDATED );
    let mut outdated_packages_file = fs::File::create(outdated_packages_file_path)?;

    outdated_packages_file.write_all( CHAR_EMPTY.as_bytes() )?;

    Ok(())
}

fn refresh_packages( be_quiet: bool ) {
    let package_info: Vec<(String, String)>                  = get_package_info();
    let mut outdated_packages: Vec<(String, String, String)> = Vec::new();

    let mut package_name: String;
    let mut package_current_version: String;
    let mut package_latest_version: String;
    let exit_message: String;

    if ! be_quiet {
        println!( "\n {arrow} {action} AUR database\n", arrow = make_blue(CHAR_ARROW), action = make_blue(ACTION_REFRESHING) );
    }
    
    for info_tuple in package_info {
        package_name            = info_tuple.0;
        package_current_version = info_tuple.1;

        if ! be_quiet {
            print!("\r{}", OUTPUT_CLEANER); // Clean output line
            print!("\r {arrow} {action} {package_name}..."
                  , arrow        = make_blue(CHAR_ARROW)
                  , action       = make_blue(ACTION_CHECKING)
                  , package_name = make_bold(&package_name));

            flush_stout();
        }

        package_latest_version = get_latest_package_version(&package_name);

        if package_current_version != package_latest_version {
            let mut is_newer_version: bool = false;

            let current_version_segments: Vec<&str> = package_current_version.split(CHAR_DOT).collect();
            let latest_version_segments: Vec<&str>  = package_latest_version.split(CHAR_DOT).collect();
            
            // Since some packages get pulled directly from their repos it its necesary to check if they are actually a newer version
            for ( i, _ ) in current_version_segments.iter().enumerate() {
                if current_version_segments[i] > latest_version_segments[i] {
                    is_newer_version = true;
                    break;
                }
            }
            
            if ! is_newer_version {
                if ! be_quiet {
                    println!( "\r {arrow} {action} available for {package_name} from {old} to {new}"
                          , arrow        = make_red(CHAR_ARROW)
                          , action       = make_red(ACTION_UPDATE)
                          , package_name = make_bold(&package_name)
                          , old          = package_current_version
                          , new          = make_bold(&package_latest_version) );
                }

                let package_info_tuple: (String, String, String) = (package_name, package_current_version, package_latest_version);

                outdated_packages.push(package_info_tuple);
            }
        }
    }

    if outdated_packages.len() > 0 {
        println!();
        exit_message = format!( "run {} to install available updates", make_yellow("'rpm -Su'") );
        let _        = write_to_outdated_packages_file(outdated_packages);

    } else {
        exit_message = format!( "everything is up to date" );
        let _        = empty_outdated_packages_file();
    }

    if ! be_quiet {
        println!("\r {arrow} {action} refreshing, {message}"
                , arrow   = make_green(CHAR_ARROW)
                , action  = make_green(ACTION_DONE)
                , message = make_bold(&exit_message) );
    }
}

fn show_installed_packages( be_quiet: bool, arguments: &mut Vec<String> ) {
    let package_info_directory: String = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, DIRECTORY_NAME_PACKAGE_INFO );

    let mut tabs: String;

    let mut ls_command = Command::new(COMMAND_NAME_LS);
    ls_command.arg(&package_info_directory);

    let ls_output_string: String    = assert_command_success( &mut ls_command, CODE_INFORMATION_LS );
    let ls_output_vector: Vec<&str> = ls_output_string.split(CHAR_NEWLINE).collect(); 

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
        println!( "\n{package_name}\t\t{version}", package_name = make_red(HEADER_PACKAGE_NAME), version = make_red(HEADER_PACKAGE_VERSION) );
    }

    for package_name in matching_packages.iter() {
        let package_info_file: String = format!( "{}{}", &package_info_directory, package_name );

        let mut read_pkgver_command = Command::new(COMMAND_NAME_GREP);
        read_pkgver_command.args(&COMMAND_ARGS_GREP_READ_PKGVER);
        read_pkgver_command.arg(&package_info_file);

        let mut read_pkgrel_command = Command::new(COMMAND_NAME_GREP);
        read_pkgrel_command.args( &["-oP", "(?<=pkgrel=).*", &package_info_file] ); // Can't figure out why can't use the const

        let read_pkgver_string = assert_command_success( &mut read_pkgver_command, CODE_INFORMATION_GREP_PKG_READ );
        let read_pkgrel_string = assert_command_success( &mut read_pkgrel_command, CODE_INFORMATION_GREP_PKG_READ );

        if be_quiet {
            tabs = CHAR_SPACE.to_string();
        } else {
            tabs = get_number_of_tabs(package_name);
        } 

        println!("{package_name}{number_of_tabs}{old}-{new}"
                , package_name = make_bold(package_name)
                , number_of_tabs= tabs
                , old = read_pkgver_string
                , new = read_pkgrel_string );
    }
}

fn show_outdated_packages( be_quiet: bool, arguments: &mut Vec<String> ) {
    let outdated_file_path: String = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, FILE_NAME_OUTDATED );
    let mut version_spacer: String;
    let mut tabs: String;

    let mut cat_command = Command::new(COMMAND_NAME_CAT);
    cat_command.arg(outdated_file_path);

    let cat_output_string: String = assert_command_success( &mut cat_command, CODE_INFORMATION_CAT );
    let cat_output_vector: Vec<&str> = cat_output_string.split(CHAR_NEWLINE).collect();

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
            println!( " {arrow} {action} ", arrow = make_green(CHAR_ARROW), action = make_green(ACTION_EVERYTHING_UPDATED) );
        }
        
        return
    }

    if ! be_quiet {
        println!("\n{package_name}\t\t{old}\t\t{new}"
                , package_name = make_red(HEADER_PACKAGE_NAME)
                , old = make_red(HEADER_PACKAGE_VERSION_OLD)
                , new = make_red(HEADER_PACKAGE_VERSION_NEW) );
    }
    
    for package_info in matching_packages {
        let package_info_vector: Vec<&str> = package_info.split(CHAR_SPACE).collect();
        let package_name: &str             = package_info_vector[0];
        let package_current_version: &str  = package_info_vector[1];
        let package_latest_version: &str   = package_info_vector[2];

        if be_quiet {
            tabs = CHAR_SPACE.to_string();
            version_spacer = STRING_VERSION_SPACER.to_string();
        } else {
            tabs = get_number_of_tabs(package_name);
            version_spacer = TABS_2.to_string();
        } 

        println!( "{package_name}{number_of_tabs}{old}{spacer}{new}"
                , package_name   = make_bold(package_name)
                , number_of_tabs = tabs
                , old            = package_current_version
                , spacer         = version_spacer
                , new            = package_latest_version );
    }
}

fn update_outdated_packages( be_quiet: bool ) {
    let outdated_file_path = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, FILE_NAME_OUTDATED );

    let mut cat_command = Command::new(COMMAND_NAME_CAT);
    cat_command.arg(outdated_file_path);

    let cat_output_string: String    = assert_command_success( &mut cat_command, CODE_INFORMATION_CAT );
    let outdated_packages: Vec<&str> = cat_output_string.split(CHAR_NEWLINE).collect();

    if outdated_packages[0] == "" {
        if ! be_quiet {
            println!( " {arrow} {action}", arrow = make_green(CHAR_ARROW), action = make_green(ACTION_NO_UPDATES) );
        }

        return
    }
    
    for package_info in outdated_packages {
        let package_info_vector: Vec<&str> = package_info.split(CHAR_SPACE).collect();
        let package_name: &str             = package_info_vector[0];
        let package_current_version: &str  = package_info_vector[1];
        let package_latest_version: &str   = package_info_vector[2];

        println!( " {arrow} {action} {package_name} from {old} to {new}"
                , arrow        = make_blue(CHAR_ARROW)
                , action       = make_blue(ACTION_UPDATING)
                , package_name = make_bold(package_name)
                , old          = package_current_version
                , new          = make_bold(package_latest_version) );
        
        sync_package(package_name, be_quiet);

        println!(" {arrow} {action} updating {package_name}"
                , arrow        = make_green(CHAR_ARROW)
                , action       = make_green(ACTION_DONE)
                , package_name = make_bold(package_name) );
    }

    let _ = empty_outdated_packages_file();

    println!( "\n {arrow} {action}", arrow = make_green(CHAR_ARROW), action = make_green(ACTION_DONE_UPDATING) );
}

fn show_update_manager() {
    let outdated_file_path = format!( "{}{}{}", get_home_directory(), DIRECTORY_NAME_RPM_CONFIG, FILE_NAME_OUTDATED );

    let mut cat_command = Command::new(COMMAND_NAME_CAT);
    cat_command.arg(outdated_file_path);

    let cat_output_string: String = assert_command_success( &mut cat_command, CODE_INFORMATION_CAT );
    let outdated_packages: Vec<&str> = cat_output_string.split(CHAR_NEWLINE).collect();

    let manager_title_string: String = center_text(PROGRAM_NAME);
    
    println!( "{}", make_green(&manager_title_string) );

    if outdated_packages[0] == "" {
        println!( "{}\n", make_blue( &center_text(TITLE_NO_UPDATES) ) );
        get_user_input( &format!( " {} {} ", make_red(ACTION_QUIT), make_red(CHAR_ARROW) ) );

        return

    } else {
        let available_updates_string: String;

        if outdated_packages.len() == 1 {
            available_updates_string = center_text(TITLE_ONE_UPDATE);
        } else {
            available_updates_string = center_text( &format!( "{} Updates available", outdated_packages.len() ) );
        }
    
        println!( "{}\n", make_yellow(&available_updates_string) );
    } 
    
    for package_info in outdated_packages {
        let package_info_vector: Vec<&str> = package_info.split(CHAR_SPACE).collect();

        println!( " {arrow} {package_name} {old} to {new}"
                , arrow        = make_red(CHAR_ARROW)
                , package_name = make_red(package_info_vector[0])
                , old          = package_info_vector[1]
                , new          = make_bold(package_info_vector[2]) );
    }

    let user_input: &str = &get_user_input( &format!( "\n {} or {} {} ", make_green(ACTION_INSTALL), make_red(ACTION_QUIT), make_red(CHAR_ARROW) ) );
    match user_input {
        "Quit" | "quit" | "N" | "n" | "No" | "no" | "Q" | "q" | "" => (),
        "Install" | "install" | "Y" | "y" | "Yes" | "yes" | "I" | "i" => update_outdated_packages(true),
        _ => exit_with_error_code(0xB1),
    }
}

fn read_environmental_arguments( arguments: &mut Vec<String>, quiet_mode_enabled: &mut bool ) {
    if arguments.len() > 1 {
        let first_argument: &str = &arguments[1];

        if first_argument.starts_with(CHAR_DASH) {
            match first_argument {
                "-h" | "--help"    => print_help_message(),

                "-V" | "--version" => println!( " {arrow} {program_name} version {version}"
                                              , arrow        = make_green(CHAR_ARROW)
                                              , program_name = make_green(PROGRAM_NAME)
                                              , version      = make_bold(PROGRAM_VERSION) ),

                "-Q" | "--query"   => show_installed_packages(*quiet_mode_enabled, arguments),

                "-Qu"              => show_outdated_packages(*quiet_mode_enabled, arguments),

                "-M" | "--manage"  => show_update_manager(),

                "-R" | "--remove"  => remove_package( &arguments[2], *quiet_mode_enabled ), 

                "-S" | "--sync"    => { for package in arguments[2..].iter() {
                                            sync_package( package, *quiet_mode_enabled );
                                      } },

                "-Su"              => update_outdated_packages(*quiet_mode_enabled),

                "-Sy"              => refresh_packages(*quiet_mode_enabled),

                "-Syu"             => { refresh_packages(*quiet_mode_enabled);
                                          update_outdated_packages(*quiet_mode_enabled); },

                "-Syqu"            => { refresh_packages(*quiet_mode_enabled);
                                        show_outdated_packages(*quiet_mode_enabled, arguments); }

                "-q" | "--quiet"   => { *quiet_mode_enabled = true;
                                          arguments.remove(0); // Remove the processed argument
                                          read_environmental_arguments( arguments, quiet_mode_enabled); },

                "-E" | "--explain" => { let error_code = i32::from_str_radix(&arguments[2], 16);
                                        explain_error_code( error_code.unwrap() ); },

                _                  => println!( " {arrow} {action} option {option}, run {command} for help "
                                              , arrow   = make_red(CHAR_ARROW)
                                              , action  = make_red(ACTION_INVALID)
                                              , option  = make_red(first_argument)
                                              , command = make_yellow("'rpm --help'") ),
            }

        } else {
            sync_package( first_argument, *quiet_mode_enabled );
        }

    } else {
        println!( " {arrow} {action}, you need to specify at least a {argument} for installation"
                , arrow    = make_red(CHAR_ARROW)
                , action   = make_red(ACTION_NO_ARGUMENTS)
                , argument = make_yellow(STRING_PACKAGE_NAME) );
    }
}

fn main() {
    let mut quiet_mode_enabled: bool = false;
    let mut args: Vec<String> = env::args().collect();

    create_necessary_directories();

    read_environmental_arguments( &mut args, &mut quiet_mode_enabled );
}