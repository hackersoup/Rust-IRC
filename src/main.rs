use std::string::String;
use std::option::Option;
use std::str::FromStr;
use std::num::ParseIntError;

struct Client {
    nickname: String,       // Unique ID for the client. Max len 9
    hostname: String,       // Hostname of the client's computer
    username: String,       // Username of client on their computer
    current_server: String, // Name of the server the client is connected to
    permissions: String,    // The permissions associated with the client for a particular channel
}

struct Channel {
    name: String,         // Starts with & or #, max 200 bytes. No space, ASCII 7, or comma
    clients: Vec<Client>, // List of clients connected to the channel
}

// Max size of 510 plus CRLF
// Cannot contain NUL
#[derive(Debug)]
struct Message {
    prefix: Option<String>,
    command: Option<Command>,
    parameters: Option<String>,
}


#[derive(Debug)]
enum Command {
    Pass, None
}

enum ChannelType {
    DISTRIBUTED,
}

enum OpOnlyCommands {
    KICK,
    MODE,
    INVITE,
    TOPIC,
}

enum Error {
    MESSAGE_PARSE_FAILURE
}

/* Pseudo-BNF for message format
 * See notes for gotchas and tricks.
 * * <message>  ::= [':' <prefix> <SPACE> ] <command> <params> <crlf>
 * * <prefix>   ::= <servername> | <nick> [ '!' <user> ] [ '@' <host> ]
 * * <command>  ::= <letter> { <letter> } | <number> <number> <number>
 * * <SPACE>    ::= ' ' { ' ' }
 * * <params>   ::= <SPACE> [ ':' <trailing> | <middle> <params> ]
 * *
 * * <middle>   ::= <Any *non-empty* sequence of octets not including SPACE
 * *                or NUL or CR or LF, the first of which may not be ':'>
 * * <trailing> ::= <Any, possibly *empty*, sequence of octets not including
 * *                  NUL or CR or LF>
 * *
 * * <crlf>     ::= CR LF
*/
fn parse_message(message: String) {
    // Prefix messages start with a : and no space between the : and the prefix
    // TODO Handle prefixes
    let mut tokens = message.split(' ');

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
                },
                None => None
            }
        },
        None => {
            None
        },
    };

    // Set the command as the next token if there was a prefix,
    // or the first token if there was no prefix
    let command = match prefix {
        Some(_) => tokens.next(),
        None => first
    };
    println!("{:?}\t{:?}", prefix, command);
    

}

fn main() {
    parse_message(String::from(":HELLO WORLD"));
    parse_message(String::from("HELLO WORLD"));
}
