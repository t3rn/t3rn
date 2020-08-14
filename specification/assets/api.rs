/**
    Messages
**/


/**
    Types
**/

struct Package {

}

struct Standard {

}

struct Dest {

}

struct Source {

}

struct Gateway {

}

struct Circuit {

}

/**
    Type of a request for the network (Circuit) to process.
    New - creates new entities (standards, packages, chains etc.)
    Update - updates the entities
    Exec - executes registered standards/packages on specified chains.
**/
enum RequestType {
    New,
    Update,
    Exec,
}

struct ExecMeta {
    size: u64,
    last_updated: Date,

}

struct Exec {
    check: Binaries,
    check: Binaries,
}
struct Binaries {
    meta: *u8,
    exec: *u8,
}

struct Result {

}

struct Step {
    dest: Chain,
    result: Result,
    exec: Exec,
    binaries: *u8,
}

struct Meta {
    steps: Vec<Step>
}

struct Request {
    req_type: RequestType,

}

struct Response {

}

struct Result {

}


/**
    Meta description of a new package
**/
struct Meta {

}
struct MsgIn {

}