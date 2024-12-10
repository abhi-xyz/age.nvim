#![deny(clippy::unwrap_used)]

use std::{fs, path};

use nvim_oxi::api::opts::BufDeleteOpts;
use nvim_oxi::{print, Dictionary, Result as OxiResult};

use crate::command::Command;
use crate::crypt::decrypt_file;
use crate::error::JustError;
use crate::{crypt::encrypt_file, config::Config};

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    /// Creates a new `App` instance with the provided configuration.
    ///
    /// This function initializes the application state with the specified `Config`.
    pub fn new(config: Config) -> Self {
        App { config }
    }

    /// Sets up the application with the provided options from a `Dictionary`.
    ///
    /// This function allows the plugin to be reconfigured dynamically, using
    /// a dictionary of options passed from Neovim.
    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    /// Handles commands issued to the plugin.
    ///
    /// Based on the command and argument passed, the corresponding action (such as
    /// setting the font or closing the window) is performed.
    pub fn handle_command(&mut self, cmd: Command) -> OxiResult<()> {
        match cmd {
            Command::DecryptFile => {
                let re = self.decrypt_current_file();
                if let Err(err) = re {
                    print!("{}", err);
                }
                Ok(())
            }
            Command::EncryptFile => {
                let re = self.encrypt_current_file();
                if let Err(err) = re {
                    print!("{}", err);
                }
                Ok(())
            }
            Command::NewFileName(Some(arg)) => {
                let re = self.open_age_file(arg);
                if let Err(err) = re {
                    print!("{}", err);
                }
                Ok(())
            }
            Command::NewFileName(None) => Ok(()), // self.open_age_file(),
        }
    }

    /// Just new (open) filename.some
    /// opens a new file if it doesnot exist
    /// else if the file.ends with .age it will decrypt_file and open
    fn open_age_file(&mut self, arg: String) -> Result<(), JustError> {
        // got pub Key
        // got priv Key
        // got a way to get file path.
        // quit the buffer and switch to an existing revious one or new Buffer
        // then encrypt the file
        let binding = self.config.public_key.to_string();
        let public_key = binding.as_str();
        let prv_binding = self.config.private_key.to_string();
        let private_key = prv_binding.as_str();
        let args = arg.trim();

        let binding = nvim_oxi::api::get_current_buf().get_name()?;
        let cfile = binding.to_string_lossy();
        print!("{cfile}");

        if !args.is_empty() {
            nvim_oxi::api::command(&format!("edit {}", args))?;
        } else {
            print!("got empty string");
        }

        // let input_path = Path::new("new.md");
        // let encrypted_path = Path::new("new.md.age");
        // let decrypted_path = Path::new("new_decrypted.md");

        // Encrypt the file
        // encrypt_file(input_path, encrypted_path, public_key).unwrap();

        // Decrypt the file
        // decrypt_file(encrypted_path, decrypted_path, private_key).unwrap();
        // let input_result = api::input("Enter filename: ").unwrap();

        // let input_string = format!("{}", input_result);
        // print!("{input_string}");

        // match input_result {
        //     Ok(input) => {
        //         if input.to_string().is_empty() || input.to_string() == "" {
        //             print!("No filename provided.");
        //         } else {
        //            let name = api::create_namespace(input.to_string().as_str());
        //            api::command(&format!("edit {}", name)).unwrap();
        //         }
        //     },
        //     Err(err) => { print!("{err}"); },
        // }
        print!("{args}");
        print!("public_key: {public_key}\nprivate_key: {private_key}");
        Ok(())
    }

    fn decrypt_current_file(&self) -> Result<(), JustError> {
        let binding = self.config.private_key.to_string();
        let private_key = binding.as_str();
        let current_file_bufnr = nvim_oxi::api::get_current_buf();
        let current_file_path = current_file_bufnr.get_name()?;
        let current_file = current_file_path.to_string_lossy();
        let extension = current_file_path
            .extension()
            .map(|e| e.to_string_lossy().to_string());
        match extension {
            Some(ext) if ext == "age" => {
                let name = current_file.rsplit_once(".");
                if let Some((f, e)) = name {
                    let new_filename = f;
                    let age_extension = e;
                    if age_extension != "age" {
                        print!("returned early Ok");
                        return Ok(());
                    } else {
                        if path::Path::new(new_filename).exists() {
                            fs::remove_file(new_filename)?;
                        }
                        let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
                        nvim_oxi::api::set_current_buf(&new_scratch_buf)?;
                        print!(
                            "input: {}\noutput: {}\nprivate_key: {}",
                            &current_file_path.display(),
                            path::Path::new(new_filename).display(),
                            private_key
                        );
                        let opts = BufDeleteOpts::builder()
                            .force(true) // Force deletion, ignoring unsaved changes
                            .build();
                        nvim_oxi::api::Buffer::delete(current_file_bufnr, &opts)?;
                        let result = decrypt_file(
                            &current_file_path,
                            path::Path::new(new_filename),
                            private_key,
                        );
                        match result {
                            Ok(_) => {
                                let command = format!("edit {}", new_filename);
                                nvim_oxi::api::command(&command)?;
                            }
                            Err(err) => print!("{}", err),
                        }
                    }
                }
            }
            Some(_) => {
                print!("was it encrypted?");
            }
            None => {
                print!("seriously? This file have no extension.");
            }
        }
        Ok(())
    }

    fn encrypt_current_file(&self) -> Result<(), JustError> {
        let binding_pub = self.config.public_key.to_string();
        let public_key = binding_pub.as_str();
        let prv_binding = self.config.private_key.to_string();
        let _private_key = prv_binding.as_str();
        let binding_pri = nvim_oxi::api::get_current_buf().get_name()?;
        let cfile = binding_pri.to_string_lossy();
        let list_buf = nvim_oxi::api::list_bufs();
        let d = list_buf.len();
        // if len is one will will create a new buf
        if d == 1 {
            // is a scrach buf may be we can show some
            let new_scratch_buf = nvim_oxi::api::create_buf(false, true)?;
            nvim_oxi::api::set_current_buf(&new_scratch_buf)?;
        } else {
            for buf in list_buf {
                if buf.get_name()?.to_string_lossy() != cfile {
                    nvim_oxi::api::set_current_buf(&buf)?;
                    break;
                }
            }
        }
        let binding = cfile.to_string();
        let extension_result = path::Path::new(&binding).extension();
        match extension_result {
            Some(ext) => {
                let new_extension = ext.to_string_lossy().to_string() + ".age";
                let result = encrypt_file(
                    path::Path::new(&cfile.to_string()),
                    &path::Path::new(&cfile.to_string()).with_extension(new_extension),
                    public_key,
                );
                match result {
                    Ok(_) => {
                        fs::remove_file(binding_pri)?;
                    }
                    Err(err) => print!("{}", err),
                }
            }
            None => {
                encrypt_file(
                    path::Path::new(&cfile.to_string()),
                    &path::Path::new(&cfile.to_string()).with_extension("age"),
                    public_key,
                )?;
            }
        }
        Ok(())
    }
}