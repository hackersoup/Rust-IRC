mod irc {
    mod errors {
        #[derive(Debug)]
        pub enum InternalError {
            MessageParseFailure,
            NotEnoughParameters,
            NoCommandGiven,
            NoSuchCommand,
        }

        enum ResponseError {
            NeedMoreParams,
            AlreadyRegistered,
            NoNicknameGiven,
            ErroneousNickname,
            NicknameCollision,
        }
    }

    mod commands {
        use std::str::FromStr;
        use super::errors::InternalError;

        // Max size of 510 plus CRLF
        // Cannot contain NUL
        #[derive(Debug)]
        pub struct Message {
            prefix: Option<String>,
            command: Command,
        }

        /// The possible commands that can be issued to an IRC server
        #[derive(Debug)]
        enum Command {
            Pass {
                password: String,
            },
            Nick {
                nickname: String,
                hopcount: Option<String>,
            },
            User {
                username: String,
                hostname: String,
                servername: String,
                realname: String,
            },
        }

        /// Parse a string into an IRC message
        impl FromStr for Message {
            type Err = super::errors::InternalError;

            fn from_str(s: &str) -> Result<Message, Self::Err> {
                let mut tokens = s.split_whitespace();

                // TODO This is spaghetti and makes me sad. Clean it up
                let first = tokens.next();
                let prefix = match first {
                    Some(possible_prefix) => {
                        // We have data, now to validate that it is a prefix starting with :
                        // TODO Only get the part of the string AFTER the :, not including it
                        let mut possible_prefix_iter = possible_prefix.chars();
                        match possible_prefix_iter.next() {
                            Some(c) => {
                                if c == ':' {
                                    Some(String::from(possible_prefix))
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    None => None,
                };

                // Convert the command to uppercase for matching
                // TODO More spaghetti from previous code block / nightmare. Cleanup when possible
                let command = match prefix {
                    Some(_) => match tokens.next() {
                        Some(command) => command.to_uppercase(),
                        None => return Err(InternalError::NoCommandGiven),
                    },
                    None => match first {
                        Some(command) => command.to_uppercase(),
                        None => return Err(InternalError::NoCommandGiven),
                    },
                };

                let command: &str = &(command);
                // Main command parsing block
                // TODO Is individual command parsing the responsability of this, or should it be
                // associated to each command? Would it be better to have command structs that each are
                // capable of parsing themselves from a string so that they may throw more targeted error
                // messages on formatting issues, eg a bad nickname? Will consider for later.
                let command = match command {
                    "PASS" => match tokens.next() {
                        Some(password) => Command::Pass {
                            password: String::from(password),
                        },
                        None => return Err(InternalError::NotEnoughParameters),
                    },
                    "NICK" => match tokens.next() {
                        Some(nickname) => Command::Nick {
                            nickname: nickname.to_string(),
                            hopcount: tokens.next().map(String::from),
                        },
                        None => return Err(InternalError::NotEnoughParameters),
                    },
                    "USER" => match tokens.next() {
                        Some(username) => match tokens.next() {
                            Some(hostname) => match tokens.next() {
                                Some(servername) => match tokens.next() {
                                    Some(realname) => Command::User {
                                        username: username.to_string(),
                                        hostname: hostname.to_string(),
                                        servername: servername.to_string(),
                                        realname: realname.to_string(),
                                    },
                                    None => return Err(InternalError::NotEnoughParameters),
                                },
                                None => return Err(InternalError::NotEnoughParameters),
                            },
                            None => return Err(InternalError::NotEnoughParameters),
                        },
                        None => return Err(InternalError::NotEnoughParameters),
                    },
                    _ => return Err(InternalError::NoSuchCommand),
                };

                Ok(Message {
                    prefix: prefix,
                    command: command,
                })
            }
        }

        enum ChannelType {
            Distributed,
        }

        enum OpOnlyCommands {
            Kick,
            Mode,
            Invite,
            Topic,
        }

    }
    use std::option::Option;
    use std::str::FromStr;
    use std::string::String;

    struct Client {
        nickname: String,       // Unique ID for the client. Max len 9
        hostname: String,       // Hostname of the client's computer
        username: String,       // Username of client on their computer
        current_server: String, // Name of the server the client is connected to
        permissions: String, // The permissions associated with the client for a particular channel
    }

    struct Channel {
        name: String,         // Starts with & or #, max 200 bytes. No space, ASCII 7, or comma
        clients: Vec<Client>, // List of clients connected to the channel
    }

}

fn main() {}
