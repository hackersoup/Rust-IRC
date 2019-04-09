struct Client {
    nickname: String, // Unique ID for the client. Max len 9
    hostname: String, // Hostname of the client's computer
    username: String, // Username of client on their computer
    current_server: String, // Name of the server the client is connected to
    permissions: String, // The permissions associated with the client for a particular channel
}

struct Channel {
    name: String, // Starts with & or #, max 200 bytes. No space, ASCII 7, or comma
    clients: Vec<Client>, // List of clients connected to the channel
}

// Max size of 510 plus CRLF
// Cannot contain NUL
struct Message {
    prefix: String,
    command: String,
    parameters: String,
}

enum ChannelType {
    DISTRIBUTED,
}

enum OpOnlyCommands {
    KICK, MODE, INVITE, TOPIC,
}

enum Error {

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

fn parse_message(message: String) -> Message {
    // Prefix messages start with a : and no space between the : and the prefix
    
}


fn main() {

}
